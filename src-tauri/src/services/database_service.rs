use crate::claude_code::ClaudeCliProcess;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct DatabaseService;

impl DatabaseService {
    /// Reset the database by deleting the database file
    pub async fn reset_database(
        app_handle: &AppHandle,
        active_processes: Option<Arc<Mutex<HashMap<Uuid, ClaudeCliProcess>>>>,
    ) -> Result<String, String> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {e}"))?;

        let db_path = app_dir.join("agentia.db");

        if db_path.exists() {
            // Shutdown all processes first if provided
            if let Some(processes) = active_processes {
                let mut processes = processes.lock().await;
                for (_, process) in processes.drain() {
                    let _ = process.shutdown().await;
                }
            }

            std::fs::remove_file(&db_path)
                .map_err(|e| format!("Failed to delete database: {e}"))?;

            Ok(format!("Database deleted successfully at: {db_path:?}"))
        } else {
            Ok(format!("No database found at: {db_path:?}"))
        }
    }
}
