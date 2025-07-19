#![allow(clippy::used_underscore_binding)]

mod claude_code;
mod db;

use claude_code::{
    session::SessionStatus, ClaudeCliProcess, Message, MessageRole, Session, SessionConfig,
};
use db::Database;
use std::collections::HashMap;
use tauri::{Emitter, Manager};
use tokio::sync::Mutex;
use uuid::Uuid;

// Simplified application state
pub struct AppState {
    active_processes: Mutex<HashMap<Uuid, ClaudeCliProcess>>,
    db: Mutex<Database>,
    current_project_id: Mutex<Option<i32>>,
    current_worktree_id: Mutex<Option<i32>>,
}

impl AppState {
    fn new(db: Database) -> Self {
        Self {
            active_processes: Mutex::new(HashMap::new()),
            db: Mutex::new(db),
            current_project_id: Mutex::new(None),
            current_worktree_id: Mutex::new(None),
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
    // Get current project and worktree IDs
    let project_id = (*state.current_project_id.lock().await).ok_or("No project selected")?;
    let worktree_id = (*state.current_worktree_id.lock().await).ok_or("No worktree selected")?;

    // Create session
    let session = Session::new(title.clone(), config.clone());

    // Serialize config to JSON
    let config_json = serde_json::to_string(&config).map_err(|e| e.to_string())?;

    // Create session in DB
    let db = state.db.lock().await;
    let db_session = db
        .create_session(project_id, worktree_id, &title, Some(&config_json))
        .map_err(|e| e.to_string())?;

    // Return session with proper ID from DB
    let session_with_db_id =
        Session { id: Uuid::parse_str(&db_session.id).map_err(|e| e.to_string())?, ..session };

    Ok(session_with_db_id)
}

#[tauri::command]
async fn get_session(
    state: tauri::State<'_, AppState>,
    session_id: Uuid,
) -> Result<Session, String> {
    // Get session from DB
    let db = state.db.lock().await;
    let db_session = db
        .get_session(&session_id.to_string())
        .map_err(|e| e.to_string())?
        .ok_or("Session not found")?;

    // Deserialize config
    let config = if let Some(config_json) = &db_session.default_config {
        serde_json::from_str(config_json).unwrap_or_default()
    } else {
        SessionConfig::default()
    };

    // Check if process is running for this session
    let process_status = {
        let processes = state.active_processes.lock().await;
        if let Some(process) = processes.get(&session_id) {
            if let Some(exit_code) = process.get_exit_code() {
                if exit_code == 0 {
                    SessionStatus::Exited
                } else {
                    SessionStatus::Failed { exit_code }
                }
            } else {
                SessionStatus::Running
            }
        } else {
            SessionStatus::Exited
        }
    };

    let mut session = Session {
        id: session_id,
        title: db_session.title.clone(),
        config,
        messages: Vec::new(),
        status: process_status,
        created_at: db_session.created_at,
        updated_at: db_session.updated_at,
        claude_session_id: None, // Now stored in claude_cli_sessions table
    };

    // Load messages from Claude Code file - get latest CLI session
    let latest_cli_session =
        db.get_latest_claude_cli_session(&session_id.to_string()).map_err(|e| e.to_string())?;

    if let Some(cli_session) = latest_cli_session {
        if let Ok(entries) = claude_code::read_session_file(&cli_session.file_path) {
            session.messages = convert_claude_entries_to_messages(entries);
        }
        session.claude_session_id = Some(cli_session.claude_session_id);
    }

    Ok(session)
}

#[tauri::command]
async fn list_sessions(state: tauri::State<'_, AppState>) -> Result<Vec<Session>, String> {
    // Get current project ID and path
    let project_id = (*state.current_project_id.lock().await).ok_or("No project selected")?;

    // Get project path
    let db = state.db.lock().await;
    let project =
        db.get_project(project_id).map_err(|e| e.to_string())?.ok_or("Project not found")?;

    let project_path = project.path;

    // Get all Claude Code sessions for this project
    let claude_sessions = claude_code::list_existing_sessions(&project_path).unwrap_or_default();

    // Get DB sessions
    let db_sessions = db.get_sessions_by_project(project_id).map_err(|e| e.to_string())?;

    // Create a map of claude_session_id to DB session
    let mut db_session_map = std::collections::HashMap::new();
    for db_session in db_sessions {
        // Get latest CLI session for this session
        if let Ok(Some(cli_session)) = db.get_latest_claude_cli_session(&db_session.id) {
            db_session_map.insert(cli_session.file_path, db_session);
        }
    }

    // Get running process statuses
    let process_statuses = {
        let processes = state.active_processes.lock().await;
        let mut statuses = std::collections::HashMap::new();
        for (session_id, process) in processes.iter() {
            let status = if let Some(exit_code) = process.get_exit_code() {
                if exit_code == 0 {
                    SessionStatus::Exited
                } else {
                    SessionStatus::Failed { exit_code }
                }
            } else {
                SessionStatus::Running
            };
            statuses.insert(*session_id, status);
        }
        statuses
    };

    let mut result = Vec::new();

    // Process all Claude Code sessions
    for claude_session in claude_sessions {
        let file_path = claude_session.file_path.clone();

        // Check if we have this session in DB
        if let Some(db_session) = db_session_map.get(&file_path) {
            // Session exists in DB, use it
            if let Ok(session_uuid) = Uuid::parse_str(&db_session.id) {
                let config = if let Some(config_json) = &db_session.default_config {
                    serde_json::from_str(config_json).unwrap_or_default()
                } else {
                    SessionConfig::default()
                };

                let status =
                    process_statuses.get(&session_uuid).cloned().unwrap_or(SessionStatus::Exited);

                let session = Session {
                    id: session_uuid,
                    title: db_session.title.clone(),
                    config,
                    messages: Vec::new(),
                    status,
                    created_at: db_session.created_at,
                    updated_at: db_session.updated_at,
                    claude_session_id: Some(file_path.clone()),
                };
                result.push(session);
            }
        } else {
            // New Claude Code session not in DB yet
            // Create a new DB entry for it
            let title = claude_session
                .first_user_message
                .as_deref()
                .unwrap_or("Claude Code Session")
                .chars()
                .take(100)
                .collect::<String>();

            let config = SessionConfig {
                model: "claude-3-5-sonnet-20241022".to_string(),
                temperature: None,
                max_tokens: None,
                permission_mode: "default".to_string(),
                working_directory: Some(project_path.clone()),
            };

            let config_json = serde_json::to_string(&config).unwrap_or_default();

            // Create in DB
            // For now, use the main worktree (this should be improved later)
            let worktrees = db.get_worktrees_by_project(project.id).unwrap_or_default();
            if let Some(main_worktree) = worktrees.iter().find(|w| w.is_main) {
                if let Ok(db_session) =
                    db.create_session(project.id, main_worktree.id, &title, Some(&config_json))
                {
                    if let Ok(session_uuid) = Uuid::parse_str(&db_session.id) {
                        let status = process_statuses
                            .get(&session_uuid)
                            .cloned()
                            .unwrap_or(SessionStatus::Exited);

                        let session = Session {
                            id: session_uuid,
                            title,
                            config,
                            messages: Vec::new(),
                            status,
                            created_at: claude_session.start_time.unwrap_or_else(chrono::Utc::now),
                            updated_at: claude_session.start_time.unwrap_or_else(chrono::Utc::now),
                            claude_session_id: Some(file_path),
                        };
                        result.push(session);
                    }
                }
            }
        }
    }

    result.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(result)
}

/// Handle Claude process streaming messages
async fn handle_claude_process_messages(
    app_handle: tauri::AppHandle,
    session_id: Uuid,
    mut receiver: tokio::sync::mpsc::UnboundedReceiver<Message>,
) {
    let mut claude_id_saved = false;

    while let Some(msg) = receiver.recv().await {
        // Emit message to frontend
        let _ = app_handle.emit(&format!("session-{session_id}-message"), &msg);

        // Update status and save Claude session ID when we first get it
        if !claude_id_saved {
            if let Some(state) = app_handle.try_state::<AppState>() {
                let processes = state.active_processes.lock().await;
                if let Some(process) = processes.get(&session_id) {
                    if let Some(claude_session_id) = process.get_claude_session_id() {
                        println!("DEBUG: Creating Claude CLI session entry: {claude_session_id}");
                        let db = state.db.lock().await;

                        // Get session config for Claude CLI session
                        let session_config =
                            if let Ok(Some(session)) = db.get_session(&session_id.to_string()) {
                                session.default_config.unwrap_or_default()
                            } else {
                                serde_json::to_string(&SessionConfig::default()).unwrap_or_default()
                            };

                        // Find parent CLI session if any
                        let parent_cli_session_id = db
                            .get_latest_claude_cli_session(&session_id.to_string())
                            .ok()
                            .flatten()
                            .map(|s| s.id);

                        // Create Claude CLI session entry
                        if let Ok(cli_session) = db.create_claude_cli_session(
                            &session_id.to_string(),
                            &claude_session_id,
                            &claude_session_id, // Using same as file path for now
                            &session_config,
                            parent_cli_session_id,
                            process.get_pid(),
                        ) {
                            // Mark as active
                            let _ = db
                                .create_active_cli_session(&session_id.to_string(), cli_session.id);
                            claude_id_saved = true;
                        }
                    }
                }
            }
        }
    }

    // Process has ended - clean up
    println!("DEBUG: Claude process ended for session {session_id}");
    cleanup_claude_process(app_handle, session_id).await;
}

/// Clean up Claude process when it ends
async fn cleanup_claude_process(app_handle: tauri::AppHandle, session_id: Uuid) {
    if let Some(state) = app_handle.try_state::<AppState>() {
        let mut processes = state.active_processes.lock().await;
        if let Some(mut process) = processes.remove(&session_id) {
            // Check exit code
            let _ = process.is_running(); // This updates the exit code
            let exit_code = process.get_exit_code().unwrap_or(0);
            println!("DEBUG: Process exited with code: {exit_code}");

            // Update Claude CLI session exit status
            let db = state.db.lock().await;

            // Find the active CLI session and update its status
            if let Ok(active_sessions) = db.get_active_cli_sessions(&session_id.to_string()) {
                for active in active_sessions {
                    let _ = db.update_claude_cli_exit_status(active.cli_session_id, exit_code);
                    let _ = db
                        .delete_active_cli_session(&session_id.to_string(), active.cli_session_id);
                }
            }
        }
    }
}

#[tauri::command]
async fn send_message(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    session_id: Uuid,
    message: String,
) -> Result<(), String> {
    // Get session from DB to get config
    let config = {
        let db = state.db.lock().await;
        let db_session = db
            .get_session(&session_id.to_string())
            .map_err(|e| e.to_string())?
            .ok_or("Session not found")?;

        if let Some(config_json) = &db_session.default_config {
            serde_json::from_str(config_json).unwrap_or_default()
        } else {
            SessionConfig::default()
        }
    };

    // Add user message
    let user_msg = Message::new_text(MessageRole::User, message.clone());

    // Emit user message
    app.emit(&format!("session-{session_id}-message"), &user_msg).map_err(|e| e.to_string())?;

    // Check if there's already a process for this session
    let mut processes = state.active_processes.lock().await;

    if let Some(process) = processes.get(&session_id) {
        // Existing process - send message to it
        println!("DEBUG: Using existing Claude process for session {session_id}");

        process.send_message(message).map_err(|e| format!("Failed to send message: {e}"))?;

        // Touch session to update timestamp
        let db = state.db.lock().await;
        let _ = db.touch_session(&session_id.to_string());
    } else {
        // No existing process - create new one
        println!("DEBUG: Creating new Claude process for session {session_id}");

        let working_dir = config.working_directory.clone();

        match ClaudeCliProcess::spawn(session_id.to_string(), working_dir) {
            Ok((process, receiver)) => {
                // Send the initial message
                process
                    .send_message(message)
                    .map_err(|e| format!("Failed to send initial message: {e}"))?;

                // Store the process
                processes.insert(session_id, process);
                drop(processes); // Release lock before spawning task

                // Handle streaming responses
                let app_handle = app.clone();
                tokio::spawn(handle_claude_process_messages(app_handle, session_id, receiver));
            }
            Err(e) => {
                return Err(format!("Failed to start Claude process: {e}"));
            }
        }
    }

    Ok(())
}

#[tauri::command]
async fn stop_session(state: tauri::State<'_, AppState>, session_id: Uuid) -> Result<(), String> {
    // Stop long-running process if exists
    let mut processes = state.active_processes.lock().await;
    if let Some(process) = processes.remove(&session_id) {
        println!("DEBUG: Stopping Claude process for session {session_id}");
        process.shutdown().await.map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
async fn shutdown_all_processes(state: tauri::State<'_, AppState>) -> Result<(), String> {
    println!("DEBUG: Shutting down all Claude processes");

    // Stop all long-running processes
    let mut processes = state.active_processes.lock().await;
    let process_list: Vec<_> = processes.drain().collect();
    drop(processes);

    for (session_id, process) in process_list {
        println!("DEBUG: Shutting down process for session {session_id}");
        if let Err(e) = process.shutdown().await {
            eprintln!("Error shutting down process for session {session_id}: {e}");
        }
    }

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
    // Parse project ID string to i32
    let project_id_int = project_id.parse::<i32>().map_err(|_| "Invalid project ID")?;

    // Validate project exists
    let project = state.db.lock().await.get_project(project_id_int).map_err(|e| e.to_string())?;

    if project.is_none() {
        return Err("Project not found".to_string());
    }

    // Update current project
    *state.current_project_id.lock().await = Some(project_id_int);

    // Also set the main worktree as current
    let worktrees = state
        .db
        .lock()
        .await
        .get_worktrees_by_project(project_id_int)
        .map_err(|e| e.to_string())?;
    if let Some(main_worktree) = worktrees.iter().find(|w| w.is_main) {
        *state.current_worktree_id.lock().await = Some(main_worktree.id);
    }

    Ok(())
}

#[tauri::command]
async fn create_project(
    state: tauri::State<'_, AppState>,
    name: String,
    path: String,
) -> Result<db::models::Project, String> {
    let project = state.db.lock().await.create_project(&path, &name).map_err(|e| e.to_string())?;

    // Set as current project
    *state.current_project_id.lock().await = Some(project.id);

    // Create main worktree for the new project
    let worktree = state
        .db
        .lock()
        .await
        .create_worktree(project.id, &path, "main", true)
        .map_err(|e| e.to_string())?;
    *state.current_worktree_id.lock().await = Some(worktree.id);

    Ok(project)
}

#[tauri::command]
async fn reset_database(app: tauri::AppHandle) -> Result<String, String> {
    use tauri::Manager;

    // Get the app data directory
    let app_dir =
        app.path().app_data_dir().map_err(|e| format!("Failed to get app data dir: {e}"))?;

    let db_path = app_dir.join("agentia.db");

    if db_path.exists() {
        // Shutdown all processes first
        if let Some(state) = app.try_state::<AppState>() {
            let mut processes = state.active_processes.lock().await;
            for (_, process) in processes.drain() {
                let _ = process.shutdown().await;
            }
        }

        // Remove the database file
        std::fs::remove_file(&db_path).map_err(|e| format!("Failed to delete database: {e}"))?;

        Ok(format!("Database deleted successfully at: {db_path:?}"))
    } else {
        Ok(format!("No database found at: {db_path:?}"))
    }
}

/// Convert Claude Code entries to agentia messages
fn convert_claude_entries_to_messages(entries: Vec<claude_code::SessionLogEntry>) -> Vec<Message> {
    entries.into_iter().filter_map(|entry| entry.to_internal_message()).collect()
}

/// Run the Tauri application
///
/// # Panics
///
/// Panics if:
/// - Failed to initialize database
/// - Failed to get current directory
/// - Failed to create default project
/// - Failed to run Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("agentia=debug,info").init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            use tauri::Manager;

            // Check CLI arguments (only in debug builds)
            #[cfg(debug_assertions)]
            {
                use tauri_plugin_cli::CliExt;

                if let Ok(matches) = app.handle().cli().matches() {
                    // Check if reset-db was explicitly passed (occurrences > 0)
                    if let Some(reset_db_arg) = matches.args.get("reset-db") {
                        if reset_db_arg.occurrences > 0 {
                            println!("DEBUG: reset-db flag was explicitly passed");
                            // Reset database
                            let app_dir =
                                app.path().app_data_dir().expect("Failed to get app data dir");
                            let db_path = app_dir.join("agentia.db");

                            if db_path.exists() {
                                std::fs::remove_file(&db_path).expect("Failed to delete database");
                                println!("Database deleted successfully at: {db_path:?}");
                            } else {
                                println!("No database found at: {db_path:?}");
                            }

                            // Exit after reset
                            std::process::exit(0);
                        }
                    }
                }
            }
            // Initialize database
            let db = Database::new(app.handle()).expect("Failed to initialize database");

            // Create app state with DB
            let state = AppState::new(db);
            // Don't set any default project for GUI app
            // Users should explicitly create or select projects

            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_session,
            get_session,
            list_sessions,
            send_message,
            stop_session,
            shutdown_all_processes,
            get_projects,
            select_project,
            create_project,
            reset_database,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
