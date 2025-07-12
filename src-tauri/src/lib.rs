#![allow(clippy::used_underscore_binding)]

mod claude;
mod db;
mod storage;

use claude::{
    ClaudeCLI, ClaudeCommandBuilder, Message, MessageRole, OutputFormat, Session, SessionConfig,
};
use db::Database;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{Emitter, Manager};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

// Application state
pub struct AppState {
    sessions: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Session>>>>>,
    active_clis: Arc<Mutex<HashMap<Uuid, ClaudeCLI>>>,
    db: Arc<Mutex<Database>>,
    current_project_id: Arc<RwLock<Option<String>>>,
}

impl AppState {
    fn new(db: Database) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            active_clis: Arc::new(Mutex::new(HashMap::new())),
            db: Arc::new(Mutex::new(db)),
            current_project_id: Arc::new(RwLock::new(None)),
        }
    }
}

// Tauri commands
#[tauri::command]
async fn create_session(
    state: tauri::State<'_, AppState>,
    title: String,
    config: SessionConfig,
) -> Result<Session, String> {
    // Get current project ID
    let project_id = {
        let current_project = state.current_project_id.read().await;
        current_project.clone().ok_or("No project selected")?
    };

    // Create session with DB ID
    let mut session = Session::new(title.clone(), config.clone());

    // Serialize config to JSON
    let config_json = serde_json::to_string(&config).map_err(|e| e.to_string())?;

    // Create session in DB
    let db_session = state
        .db
        .lock()
        .await
        .create_session(&project_id, None, &title, Some(&config_json))
        .map_err(|e| e.to_string())?;

    session.id = Uuid::parse_str(&db_session.id).map_err(|e| e.to_string())?;

    let session_arc = Arc::new(RwLock::new(session.clone()));
    state.sessions.write().await.insert(session.id, session_arc);

    Ok(session)
}

#[tauri::command]
async fn get_session(
    state: tauri::State<'_, AppState>,
    session_id: Uuid,
) -> Result<Session, String> {
    let sessions = state.sessions.read().await;

    if let Some(session_arc) = sessions.get(&session_id) {
        Ok(session_arc.read().await.clone())
    } else {
        Err("Session not found".to_string())
    }
}

#[tauri::command]
async fn list_sessions(state: tauri::State<'_, AppState>) -> Result<Vec<Session>, String> {
    // Get current project ID
    let project_id = {
        let current_project = state.current_project_id.read().await;
        current_project.clone().ok_or("No project selected")?
    };

    // Get sessions from DB for current project
    let db_sessions =
        state.db.lock().await.get_sessions_by_project(&project_id).map_err(|e| e.to_string())?;

    // Get in-memory sessions that match DB sessions
    let sessions = state.sessions.read().await;
    let mut result = Vec::new();

    for db_session in db_sessions {
        if let Ok(session_uuid) = Uuid::parse_str(&db_session.id) {
            if let Some(session_arc) = sessions.get(&session_uuid) {
                result.push(session_arc.read().await.clone());
            }
        }
    }

    result.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(result)
}

#[tauri::command]
async fn send_message(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    session_id: Uuid,
    message: String,
) -> Result<(), String> {
    // Get session
    let sessions = state.sessions.read().await;
    let session_arc = sessions.get(&session_id).ok_or("Session not found")?.clone();

    // Add user message
    let user_msg = Message::new_text(MessageRole::User, message.clone());
    session_arc.write().await.add_message(user_msg.clone());

    // Emit user message
    app.emit(&format!("session-{session_id}-message"), &user_msg).map_err(|e| e.to_string())?;

    // Get session config
    let config = session_arc.read().await.config.clone();

    // Remove existing CLI if any (no longer needed to stop)
    {
        let mut clis = state.active_clis.lock().await;
        clis.remove(&session_id);
    }

    // Start Claude CLI
    let mut cli = ClaudeCLI::new();

    // Load existing session ID if available
    if let Some(claude_session_id) = &session_arc.read().await.claude_session_id {
        println!("Loading existing Claude session ID: {claude_session_id}");
        cli.set_session_id(claude_session_id.clone());
    } else {
        println!("No existing Claude session ID for agentia session: {session_id}");
    }

    // Build command
    let mut builder = ClaudeCommandBuilder::new()
        .prompt(&message)
        .print_mode()
        .output_format(OutputFormat::StreamJson)
        .permission_mode(&config.permission_mode)
        .verbose();

    // Resume session if we have a Claude session ID
    if let Some(claude_session_id) = cli.get_session_id() {
        builder = builder.resume(claude_session_id);
    }

    // Set working directory
    if let Some(dir) = config.working_directory {
        builder = builder.working_dir(dir);
    }

    let mut receiver =
        cli.send_message(builder).map_err(|e| format!("Failed to start Claude CLI: {e}"))?;

    // Store CLI instance
    state.active_clis.lock().await.insert(session_id, cli);

    // Handle streaming responses
    let app_handle = app.clone();
    let session_id_copy = session_id;
    let session_arc_copy = session_arc.clone();

    tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            // Add message to session
            session_arc_copy.write().await.add_message(msg.clone());

            // Emit message to frontend
            let _ = app_handle.emit(&format!("session-{session_id_copy}-message"), &msg);
        }

        // Clean up CLI when done
        // Note: We need to get state from app_handle since we're in a different task
        if let Some(state) = app_handle.try_state::<AppState>() {
            let mut clis = state.active_clis.lock().await;
            if let Some(cli) = clis.remove(&session_id_copy) {
                // Get Claude session ID from CLI and save it
                if let Some(claude_session_id) = cli.get_session_id() {
                    println!("Saving Claude session ID: {claude_session_id} for agentia session: {session_id_copy}");
                    session_arc_copy.write().await.claude_session_id =
                        Some(claude_session_id.clone());

                    // Update DB with Claude session ID
                    let _ = state
                        .db
                        .lock()
                        .await
                        .update_session_claude_id(&session_id_copy.to_string(), &claude_session_id);
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn stop_session(state: tauri::State<'_, AppState>, session_id: Uuid) -> Result<(), String> {
    let mut clis = state.active_clis.lock().await;
    clis.remove(&session_id);
    Ok(())
}

#[tauri::command]
async fn get_projects(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<db::models::Project>, String> {
    state.db.lock().await.get_projects().map_err(|e| e.to_string())
}

#[tauri::command]
async fn select_project(
    state: tauri::State<'_, AppState>,
    project_id: String,
) -> Result<(), String> {
    // Validate project exists by getting the specific session
    let project = state.db.lock().await.get_project(&project_id).map_err(|e| e.to_string())?;

    if project.is_none() {
        return Err("Project not found".to_string());
    }

    // Update current project
    *state.current_project_id.write().await = Some(project_id);

    Ok(())
}

/// Run the Tauri application
///
/// # Panics
///
/// This function will panic if:
/// - The Tauri application fails to start
/// - There's an error in the Tauri context generation
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("agentia=debug,info").init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize database
            let db = Database::new(app.handle()).expect("Failed to initialize database");

            // Check for existing projects or create default
            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            let current_path = current_dir.to_string_lossy();

            let project = db.get_project_by_path(&current_path).expect("Failed to query projects");

            let project_id = if let Some(project) = project {
                project.id
            } else {
                // Create default project for current directory
                let project = db
                    .create_project(
                        &current_path,
                        current_dir
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Default Project"),
                    )
                    .expect("Failed to create default project");
                project.id
            };

            // Create app state with DB
            let state = AppState::new(db);
            *state.current_project_id.blocking_write() = Some(project_id.clone());

            // Load existing sessions from DB
            {
                let db = state.db.blocking_lock();
                let db_sessions =
                    db.get_sessions_by_project(&project_id).expect("Failed to load sessions");

                let mut sessions = state.sessions.blocking_write();
                for db_session in db_sessions {
                    // Convert DB session to in-memory session
                    // Note: We don't have messages here - they'll be loaded from Claude Code
                    if let Ok(session_uuid) = Uuid::parse_str(&db_session.id) {
                        // Deserialize config from JSON
                        let config = if let Some(config_json) = &db_session.config {
                            serde_json::from_str::<SessionConfig>(config_json).unwrap_or(
                                SessionConfig {
                                    model: "claude-3-5-sonnet-20241022".to_string(),
                                    temperature: None,
                                    max_tokens: None,
                                    permission_mode: "default".to_string(),
                                    working_directory: None,
                                },
                            )
                        } else {
                            SessionConfig {
                                model: "claude-3-5-sonnet-20241022".to_string(),
                                temperature: None,
                                max_tokens: None,
                                permission_mode: "default".to_string(),
                                working_directory: None,
                            }
                        };

                        let session = Session {
                            id: session_uuid,
                            title: db_session.title,
                            config,
                            messages: Vec::new(),
                            status: claude::session::SessionStatus::Completed,
                            created_at: db_session.created_at,
                            updated_at: db_session.updated_at,
                            claude_session_id: db_session.claude_session_id,
                        };
                        sessions.insert(session_uuid, Arc::new(RwLock::new(session)));
                    }
                }

                let count = sessions.len();
                println!("Loaded {count} sessions from database");
            }

            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_session,
            get_session,
            list_sessions,
            send_message,
            stop_session,
            get_projects,
            select_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
