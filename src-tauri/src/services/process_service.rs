use crate::claude_code::{ClaudeCliProcess, Message, MessageRole, SessionConfig};
use crate::db::Database;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct ProcessService {
    active_processes: Arc<Mutex<HashMap<Uuid, ClaudeCliProcess>>>,
    db: Arc<Mutex<Database>>,
}

impl ProcessService {
    pub fn new(
        active_processes: Arc<Mutex<HashMap<Uuid, ClaudeCliProcess>>>,
        db: Arc<Mutex<Database>>,
    ) -> Self {
        Self { active_processes, db }
    }

    /// Send a message to a Claude process
    pub async fn send_message(
        &self,
        app_handle: &AppHandle,
        session_id: Uuid,
        message: String,
    ) -> Result<(), String> {
        // Get session config
        let config = self.get_session_config(&session_id).await?;

        // Emit user message
        let user_msg = Message::new_text(MessageRole::User, message.clone());
        app_handle
            .emit(&format!("session-{session_id}-message"), &user_msg)
            .map_err(|e| e.to_string())?;

        // Check if process exists
        let mut processes = self.active_processes.lock().await;

        if let Some(process) = processes.get(&session_id) {
            // Send to existing process
            self.send_to_existing_process(process, &session_id, message).await?;
        } else {
            // Start new process
            self.start_new_process(
                &mut processes,
                app_handle,
                session_id,
                message,
                config.working_directory,
            )?;
        }

        Ok(())
    }

    /// Stop a specific session
    pub async fn stop_session(&self, session_id: Uuid) -> Result<(), String> {
        let mut processes = self.active_processes.lock().await;
        if let Some(process) = processes.remove(&session_id) {
            process.shutdown().await.map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    /// Shutdown all active processes
    pub async fn shutdown_all(&self) -> Result<(), String> {
        let mut processes = self.active_processes.lock().await;
        let process_list: Vec<_> = processes.drain().collect();
        drop(processes);

        for (session_id, process) in process_list {
            if let Err(e) = process.shutdown().await {
                eprintln!("Error shutting down process for session {session_id}: {e}");
            }
        }

        Ok(())
    }

    /// Save Claude session ID to database
    pub async fn save_claude_session_id(
        &self,
        session_id: Uuid,
        claude_session_id: &str,
        pid: Option<i32>,
    ) -> Result<(), String> {
        // Get session config
        let session_config = self.get_session_config_string(&session_id).await;

        let db = self.db.lock().await;

        // Find parent CLI session
        let parent_cli_session_id =
            db.get_latest_claude_cli_session(&session_id.to_string()).ok().flatten().map(|s| s.id);

        // Create Claude CLI session entry
        let cli_session = db
            .create_claude_cli_session(
                &session_id.to_string(),
                claude_session_id,
                claude_session_id,
                &session_config,
                parent_cli_session_id,
                pid,
            )
            .map_err(|e| e.to_string())?;

        // Create active CLI session
        db.create_active_cli_session(&session_id.to_string(), cli_session.id)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Clean up Claude process when it ends
    pub async fn cleanup_claude_process(&self, session_id: Uuid) -> Result<(), String> {
        let mut processes = self.active_processes.lock().await;

        if let Some(mut process) = processes.remove(&session_id) {
            let _ = process.is_running();
            let exit_code = process.get_exit_code().unwrap_or(0);

            // Update CLI session exit status
            self.update_exit_status(&session_id, exit_code).await?;
        }

        Ok(())
    }

    /// Handle Claude process streaming messages
    pub async fn handle_messages(
        self: Arc<Self>,
        app_handle: AppHandle,
        session_id: Uuid,
        mut receiver: tokio::sync::mpsc::UnboundedReceiver<Message>,
    ) {
        let mut claude_id_saved = false;

        while let Some(msg) = receiver.recv().await {
            self.process_single_message(&app_handle, &session_id, &msg, &mut claude_id_saved).await;
        }

        self.handle_process_termination(session_id).await;
    }

    async fn process_single_message(
        &self,
        app_handle: &AppHandle,
        session_id: &Uuid,
        msg: &Message,
        claude_id_saved: &mut bool,
    ) {
        // Emit message to frontend
        let _ = app_handle.emit(&format!("session-{session_id}-message"), msg);

        // Save Claude session ID when first received
        if !*claude_id_saved {
            self.save_claude_id_if_needed(session_id, claude_id_saved).await;
        }
    }

    async fn save_claude_id_if_needed(&self, session_id: &Uuid, claude_id_saved: &mut bool) {
        if let Err(e) = self.try_save_claude_session_id(session_id).await {
            eprintln!("Failed to save Claude session ID: {e}");
        } else {
            *claude_id_saved = true;
        }
    }

    async fn handle_process_termination(&self, session_id: Uuid) {
        if let Err(e) = self.cleanup_claude_process(session_id).await {
            eprintln!("Failed to cleanup Claude process: {e}");
        }
    }

    // Private helper methods

    async fn get_session_config(&self, session_id: &Uuid) -> Result<SessionConfig, String> {
        let db = self.db.lock().await;
        let db_session = db
            .get_session(&session_id.to_string())
            .map_err(|e| e.to_string())?
            .ok_or("Session not found")?;

        let config = if let Some(config_json) = &db_session.default_config {
            serde_json::from_str(config_json).unwrap_or_default()
        } else {
            SessionConfig::default()
        };

        Ok(config)
    }

    async fn get_session_config_string(&self, session_id: &Uuid) -> String {
        let db = self.db.lock().await;
        if let Ok(Some(session)) = db.get_session(&session_id.to_string()) {
            session.default_config.unwrap_or_default()
        } else {
            serde_json::to_string(&SessionConfig::default()).unwrap_or_default()
        }
    }

    async fn send_to_existing_process(
        &self,
        process: &ClaudeCliProcess,
        session_id: &Uuid,
        message: String,
    ) -> Result<(), String> {
        process.send_message(message).map_err(|e| format!("Failed to send message: {e}"))?;

        // Touch session
        let db = self.db.lock().await;
        let _ = db.touch_session(&session_id.to_string());

        Ok(())
    }

    fn start_new_process(
        &self,
        processes: &mut HashMap<Uuid, ClaudeCliProcess>,
        app_handle: &AppHandle,
        session_id: Uuid,
        message: String,
        working_dir: Option<String>,
    ) -> Result<(), String> {
        match ClaudeCliProcess::spawn(session_id.to_string(), working_dir) {
            Ok((process, receiver)) => {
                process
                    .send_message(message)
                    .map_err(|e| format!("Failed to send initial message: {e}"))?;

                processes.insert(session_id, process);

                // Handle streaming in background
                let service = Arc::new(ProcessService::new(
                    Arc::clone(&self.active_processes),
                    Arc::clone(&self.db),
                ));
                let app_handle = app_handle.clone();
                tokio::spawn(async move {
                    service.handle_messages(app_handle, session_id, receiver).await;
                });

                Ok(())
            }
            Err(e) => Err(format!("Failed to start Claude process: {e}")),
        }
    }

    async fn try_save_claude_session_id(&self, session_id: &Uuid) -> Result<(), String> {
        let processes = self.active_processes.lock().await;

        if let Some(process) = processes.get(session_id) {
            if let Some(claude_session_id) = process.get_claude_session_id() {
                let pid = process.get_pid();
                drop(processes); // Release lock before calling save

                self.save_claude_session_id(*session_id, &claude_session_id, pid).await?;
            }
        }

        Ok(())
    }

    async fn update_exit_status(&self, session_id: &Uuid, exit_code: i32) -> Result<(), String> {
        let db = self.db.lock().await;

        let active_sessions =
            db.get_active_cli_sessions(&session_id.to_string()).map_err(|e| e.to_string())?;

        for active in active_sessions {
            db.update_claude_cli_exit_status(active.cli_session_id, exit_code)
                .map_err(|e| e.to_string())?;
            db.delete_active_cli_session(&session_id.to_string(), active.cli_session_id)
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}

impl Clone for ProcessService {
    fn clone(&self) -> Self {
        Self { active_processes: Arc::clone(&self.active_processes), db: Arc::clone(&self.db) }
    }
}
