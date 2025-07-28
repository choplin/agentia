#![allow(clippy::used_underscore_binding)]

mod claude_code;
mod commands;
mod db;
mod error;
mod services;

use claude_code::ClaudeCliProcess;
use db::Database;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

// Simplified application state
pub struct AppState {
    pub active_processes: Arc<Mutex<HashMap<Uuid, ClaudeCliProcess>>>,
    pub db: Arc<Mutex<Database>>,
}

impl AppState {
    fn new(db: Database) -> Self {
        Self {
            active_processes: Arc::new(Mutex::new(HashMap::new())),
            db: Arc::new(Mutex::new(db)),
        }
    }
}

// Process management commands

#[tauri::command]
async fn send_message(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    session_id: Uuid,
    message: String,
) -> Result<(), String> {
    let service =
        services::ProcessService::new(Arc::clone(&state.active_processes), Arc::clone(&state.db));
    service.send_message(&app, session_id, message).await
}

#[tauri::command]
async fn stop_session(state: tauri::State<'_, AppState>, session_id: Uuid) -> Result<(), String> {
    let service =
        services::ProcessService::new(Arc::clone(&state.active_processes), Arc::clone(&state.db));
    service.stop_session(session_id).await
}

#[tauri::command]
async fn shutdown_all_processes(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let service =
        services::ProcessService::new(Arc::clone(&state.active_processes), Arc::clone(&state.db));
    service.shutdown_all().await
}

#[tauri::command]
async fn reset_database(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    services::DatabaseService::reset_database(&app, Some(Arc::clone(&state.active_processes))).await
}

/// Initialize tracing for the application
fn init_tracing() {
    tracing_subscriber::fmt().with_env_filter("agentia=debug,info").init();
}

/// Handle CLI arguments in debug mode
#[cfg(debug_assertions)]
fn handle_debug_cli_args(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri_plugin_cli::CliExt;

    if let Ok(matches) = app.handle().cli().matches() {
        if should_reset_database(&matches) {
            reset_database_from_cli(app)?;
            std::process::exit(0);
        }
    }
    Ok(())
}

#[cfg(debug_assertions)]
fn should_reset_database(matches: &tauri_plugin_cli::Matches) -> bool {
    matches.args.get("reset-db").is_some_and(|arg| arg.occurrences > 0)
}

#[cfg(debug_assertions)]
fn reset_database_from_cli(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::Manager;

    let app_dir = app.path().app_data_dir()?;
    let db_path = app_dir.join("agentia.db");

    if db_path.exists() {
        std::fs::remove_file(&db_path)?;
        println!("Database deleted successfully at: {db_path:?}");
    } else {
        println!("No database found at: {db_path:?}");
    }

    Ok(())
}

/// Initialize the application state
fn init_app_state(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::Manager;

    let db = Database::new(app.handle())?;
    let state = AppState::new(db);
    app.manage(state);

    Ok(())
}

/// Setup function for Tauri app
fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    handle_debug_cli_args(app)?;

    init_app_state(app)?;

    Ok(())
}

/// Run the Tauri application
///
/// # Panics
///
/// This function will panic if:
/// - The Tauri application fails to build or run
/// - Required plugins fail to initialize
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_tracing();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cli::init())
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![
            // Commands from modules
            commands::session::create_session,
            commands::session::get_session,
            commands::session::list_sessions,
            commands::project::get_projects,
            commands::project::create_project,
            // Process management commands
            send_message,
            stop_session,
            shutdown_all_processes,
            reset_database,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
