use anyhow::Result;
use rusqlite::Connection;
use tauri::{AppHandle, Manager};

pub mod claude_cli_session;
pub mod migrations;
pub mod models;
pub mod project;
pub mod session;
pub mod worktree;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        let app_dir = app_handle.path().app_data_dir().expect("Failed to get app data dir");

        std::fs::create_dir_all(&app_dir)?;

        let db_path = app_dir.join("agentia.db");
        let conn = Connection::open(db_path)?;

        let mut db = Self { conn };
        db.init_schema()?;

        Ok(db)
    }

    fn init_schema(&mut self) -> Result<()> {
        migrations::run_migrations(&self.conn)?;
        Ok(())
    }

    // Convenience methods that delegate to modules

    // Project methods
    pub fn create_project(&self, path: &str, name: &str) -> Result<models::Project> {
        project::create(&self.conn, path, name)
    }

    pub fn get_projects(&self) -> Result<Vec<models::Project>> {
        project::find_all(&self.conn)
    }

    #[allow(dead_code)]
    pub fn get_project_by_path(&self, path: &str) -> Result<Option<models::Project>> {
        project::find_by_path(&self.conn, path)
    }

    pub fn get_project(&self, id: i32) -> Result<Option<models::Project>> {
        project::find_by_id(&self.conn, id)
    }

    // Worktree methods
    pub fn create_worktree(
        &self,
        project_id: i32,
        path: &str,
        name: &str,
        is_main: bool,
    ) -> Result<models::Worktree> {
        worktree::create(&self.conn, project_id, path, name, is_main)
    }

    pub fn get_worktrees_by_project(&self, project_id: i32) -> Result<Vec<models::Worktree>> {
        worktree::find_by_project(&self.conn, project_id)
    }

    #[allow(dead_code)]
    pub fn get_worktree(&self, id: i32) -> Result<Option<models::Worktree>> {
        worktree::find_by_id(&self.conn, id)
    }

    #[allow(dead_code)]
    pub fn get_worktree_by_path(&self, path: &str) -> Result<Option<models::Worktree>> {
        worktree::find_by_path(&self.conn, path)
    }

    // Session methods
    pub fn create_session(
        &self,
        project_id: i32,
        worktree_id: i32,
        title: &str,
        default_config: Option<&str>,
    ) -> Result<models::Session> {
        session::create(&self.conn, project_id, worktree_id, title, default_config)
    }

    pub fn get_sessions_by_project(&self, project_id: i32) -> Result<Vec<models::Session>> {
        session::find_by_project(&self.conn, project_id)
    }

    #[allow(dead_code)]
    pub fn get_sessions_by_worktree(&self, worktree_id: i32) -> Result<Vec<models::Session>> {
        session::find_by_worktree(&self.conn, worktree_id)
    }

    pub fn get_session(&self, session_id: &str) -> Result<Option<models::Session>> {
        session::find_by_id(&self.conn, session_id)
    }

    pub fn touch_session(&self, session_id: &str) -> Result<()> {
        session::touch(&self.conn, session_id)
    }

    // Claude CLI Session methods
    pub fn create_claude_cli_session(
        &self,
        session_id: &str,
        claude_session_id: &str,
        file_path: &str,
        config: &str,
        parent_cli_session_id: Option<i32>,
        process_pid: Option<i32>,
    ) -> Result<models::ClaudeCliSession> {
        claude_cli_session::create(
            &self.conn,
            session_id,
            claude_session_id,
            file_path,
            config,
            parent_cli_session_id,
            process_pid,
        )
    }

    pub fn update_claude_cli_exit_status(&self, cli_session_id: i32, exit_code: i32) -> Result<()> {
        claude_cli_session::update_exit_status(&self.conn, cli_session_id, exit_code)
    }

    #[allow(dead_code)]
    pub fn get_claude_cli_sessions(
        &self,
        session_id: &str,
    ) -> Result<Vec<models::ClaudeCliSession>> {
        claude_cli_session::find_by_session(&self.conn, session_id)
    }

    pub fn get_latest_claude_cli_session(
        &self,
        session_id: &str,
    ) -> Result<Option<models::ClaudeCliSession>> {
        claude_cli_session::find_latest_by_session(&self.conn, session_id)
    }

    // Active CLI Session methods
    pub fn create_active_cli_session(&self, session_id: &str, cli_session_id: i32) -> Result<()> {
        claude_cli_session::create_active(&self.conn, session_id, cli_session_id)
    }

    pub fn delete_active_cli_session(&self, session_id: &str, cli_session_id: i32) -> Result<()> {
        claude_cli_session::delete_active(&self.conn, session_id, cli_session_id)
    }

    pub fn get_active_cli_sessions(
        &self,
        session_id: &str,
    ) -> Result<Vec<models::ActiveCliSession>> {
        claude_cli_session::find_active_by_session(&self.conn, session_id)
    }

    #[allow(dead_code)]
    pub fn has_active_cli_sessions(&self, session_id: &str) -> Result<bool> {
        claude_cli_session::has_active_sessions(&self.conn, session_id)
    }
}
