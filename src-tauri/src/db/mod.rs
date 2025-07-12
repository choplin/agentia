use anyhow::Result;
use rusqlite::Connection;
use tauri::{AppHandle, Manager};

pub mod migrations;
pub mod models;
pub mod repositories;

use repositories::{ProjectRepository, SessionRepository};

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

    pub fn create_project(&self, path: &str, name: &str) -> Result<models::Project> {
        let repo = ProjectRepository::new(&self.conn);
        repo.create(path, name)
    }

    pub fn get_projects(&self) -> Result<Vec<models::Project>> {
        let repo = ProjectRepository::new(&self.conn);
        repo.find_all()
    }

    pub fn get_project_by_path(&self, path: &str) -> Result<Option<models::Project>> {
        let repo = ProjectRepository::new(&self.conn);
        repo.find_by_path(path)
    }

    pub fn get_project(&self, id: &str) -> Result<Option<models::Project>> {
        let repo = ProjectRepository::new(&self.conn);
        repo.find_by_id(id)
    }

    pub fn create_session(
        &self,
        project_id: &str,
        claude_session_id: Option<&str>,
        title: &str,
        config: Option<&str>,
    ) -> Result<models::Session> {
        let repo = SessionRepository::new(&self.conn);
        repo.create(project_id, claude_session_id, title, config)
    }

    pub fn update_session_claude_id(
        &self,
        session_id: &str,
        claude_session_id: &str,
    ) -> Result<()> {
        let repo = SessionRepository::new(&self.conn);
        repo.update_claude_session_id(session_id, claude_session_id)
    }

    pub fn get_sessions_by_project(&self, project_id: &str) -> Result<Vec<models::Session>> {
        let repo = SessionRepository::new(&self.conn);
        repo.find_by_project(project_id)
    }

    #[allow(dead_code)]
    pub fn get_session(&self, session_id: &str) -> Result<Option<models::Session>> {
        let repo = SessionRepository::new(&self.conn);
        repo.find_by_id(session_id)
    }
}
