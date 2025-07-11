#![allow(clippy::used_underscore_binding)]

mod claude;

use claude::{ClaudeCLI, Message, MessageRole, Session, SessionConfig};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{Emitter, Manager};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

// Application state
pub struct AppState {
    sessions: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Session>>>>>,
    active_clis: Arc<Mutex<HashMap<Uuid, ClaudeCLI>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            active_clis: Arc::new(Mutex::new(HashMap::new())),
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
    let session = Session::new(title, config);
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
    let sessions = state.sessions.read().await;
    let mut result = Vec::new();

    for session_arc in sessions.values() {
        result.push(session_arc.read().await.clone());
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

    // Start Claude CLI
    let mut cli = ClaudeCLI::new();
    let mut receiver = cli
        .start(&message, &config.permission_mode, config.working_directory)
        .map_err(|e| format!("Failed to start Claude CLI: {e}"))?;

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
            clis.remove(&session_id_copy);
        }
    });

    Ok(())
}

#[tauri::command]
async fn stop_session(state: tauri::State<'_, AppState>, session_id: Uuid) -> Result<(), String> {
    let mut clis = state.active_clis.lock().await;

    if let Some(mut cli) = clis.remove(&session_id) {
        cli.stop().await.map_err(|e| e.to_string())?;
    }

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
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            create_session,
            get_session,
            list_sessions,
            send_message,
            stop_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
