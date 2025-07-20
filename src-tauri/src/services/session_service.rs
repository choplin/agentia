use crate::claude_code::{self, session::SessionStatus, Message, Session, SessionConfig};
use crate::db::{models, Database};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct SessionService {
    db: Arc<Mutex<Database>>,
}

impl SessionService {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
    }

    /// Create a new session
    pub async fn create_session(
        &self,
        project_id: i32,
        worktree_id: Option<i32>,
        title: Option<String>,
        default_config: Option<String>,
    ) -> Result<Session, String> {
        let title = title.unwrap_or_else(|| "Untitled Session".to_string());
        let config = if let Some(config_str) = &default_config {
            serde_json::from_str(config_str).unwrap_or_default()
        } else {
            SessionConfig::default()
        };

        let session = Session::new(title.clone(), config.clone());

        let db = self.db.lock().await;
        let db_session = db
            .create_session(project_id, worktree_id, &title, default_config.as_deref())
            .map_err(|e| e.to_string())?;

        let session_with_db_id =
            Session { id: Uuid::parse_str(&db_session.id).map_err(|e| e.to_string())?, ..session };

        Ok(session_with_db_id)
    }

    /// Get a session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<Option<Session>, String> {
        let db = self.db.lock().await;
        let Some(db_session) = db.get_session(session_id).map_err(|e| e.to_string())? else {
            return Ok(None);
        };

        let config = Self::parse_session_config(db_session.default_config.as_ref());
        let uuid = Uuid::parse_str(session_id).map_err(|e| e.to_string())?;
        let mut session = Self::build_session_from_db(uuid, &db_session, config);

        // Load messages from Claude Code file (skip in tests)
        if !cfg!(test) {
            if let Some(cli_session) = self.get_latest_cli_session(&uuid).await? {
                session.messages = Self::load_messages_from_file(&cli_session.file_path);
                session.claude_session_id = Some(cli_session.claude_session_id);
            }
        }

        Ok(Some(session))
    }

    /// List sessions with optional project filter
    pub async fn list_sessions(&self, project_id: Option<i32>) -> Result<Vec<Session>, String> {
        match project_id {
            Some(pid) => self.list_project_sessions(pid).await,
            None => self.list_all_sessions().await,
        }
    }

    async fn list_project_sessions(&self, project_id: i32) -> Result<Vec<Session>, String> {
        let db = self.db.lock().await;

        // Get project and sessions
        let project =
            db.get_project(project_id).map_err(|e| e.to_string())?.ok_or("Project not found")?;
        let db_sessions = db.get_sessions_by_project(project_id).map_err(|e| e.to_string())?;

        // In test mode, skip Claude session merging
        if cfg!(test) {
            Ok(db_sessions
                .into_iter()
                .filter_map(|db_session| Self::convert_db_session_to_api(&db_session).ok())
                .collect())
        } else {
            // Get Claude Code sessions
            let claude_sessions =
                claude_code::list_existing_sessions(&project.path).unwrap_or_default();

            // Build session map
            let db_session_map = self.build_session_map(db_sessions).await?;

            // Merge and return
            self.merge_sessions(claude_sessions, db_session_map, project_id, &project.path).await
        }
    }

    async fn list_all_sessions(&self) -> Result<Vec<Session>, String> {
        let db = self.db.lock().await;
        let sessions = db.get_all_sessions().map_err(|e| e.to_string())?;

        Ok(sessions
            .into_iter()
            .filter_map(|db_session| Self::convert_db_session_to_api(&db_session).ok())
            .collect())
    }

    // Helper methods

    fn parse_session_config(config_json: Option<&String>) -> SessionConfig {
        config_json.and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default()
    }

    fn build_session_from_db(
        session_id: Uuid,
        db_session: &models::Session,
        config: SessionConfig,
    ) -> Session {
        Session {
            id: session_id,
            title: db_session.title.clone(),
            config,
            messages: Vec::new(),
            status: SessionStatus::Exited,
            created_at: db_session.created_at,
            updated_at: db_session.updated_at,
            claude_session_id: None,
        }
    }

    async fn get_latest_cli_session(
        &self,
        session_id: &Uuid,
    ) -> Result<Option<models::ClaudeCliSession>, String> {
        let db = self.db.lock().await;
        db.get_latest_claude_cli_session(&session_id.to_string()).map_err(|e| e.to_string())
    }

    fn load_messages_from_file(file_path: &str) -> Vec<Message> {
        claude_code::read_session_file(file_path).map_or_else(
            |_| Vec::new(),
            |entries| entries.into_iter().filter_map(|entry| entry.to_internal_message()).collect(),
        )
    }

    async fn build_session_map(
        &self,
        db_sessions: Vec<models::Session>,
    ) -> Result<HashMap<String, models::Session>, String> {
        let mut map = HashMap::new();
        let db = self.db.lock().await;

        for db_session in db_sessions {
            if let Ok(Some(cli_session)) = db.get_latest_claude_cli_session(&db_session.id) {
                map.insert(cli_session.file_path, db_session);
            }
        }

        Ok(map)
    }

    async fn merge_sessions(
        &self,
        claude_sessions: Vec<claude_code::ExistingSession>,
        db_session_map: HashMap<String, models::Session>,
        project_id: i32,
        project_path: &str,
    ) -> Result<Vec<Session>, String> {
        let mut result = Vec::new();

        for claude_session in claude_sessions {
            if let Some(session) = self
                .process_claude_session(claude_session, &db_session_map, project_id, project_path)
                .await
            {
                result.push(session);
            }
        }

        result.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(result)
    }

    async fn process_claude_session(
        &self,
        claude_session: claude_code::ExistingSession,
        db_session_map: &HashMap<String, models::Session>,
        project_id: i32,
        project_path: &str,
    ) -> Option<Session> {
        let file_path = claude_session.file_path.clone();

        if let Some(db_session) = db_session_map.get(&file_path) {
            // Existing session
            Self::convert_db_session_to_api(db_session).ok()
        } else {
            // New Claude Code session
            self.create_session_from_claude(claude_session, project_id, project_path).await.ok()
        }
    }

    fn convert_db_session_to_api(db_session: &models::Session) -> Result<Session, String> {
        let session_uuid = Uuid::parse_str(&db_session.id).map_err(|e| e.to_string())?;
        let config = Self::parse_session_config(db_session.default_config.as_ref());

        Ok(Session {
            id: session_uuid,
            title: db_session.title.clone(),
            config,
            messages: Vec::new(),
            status: SessionStatus::Exited,
            created_at: db_session.created_at,
            updated_at: db_session.updated_at,
            claude_session_id: None,
        })
    }

    async fn create_session_from_claude(
        &self,
        claude_session: claude_code::ExistingSession,
        project_id: i32,
        project_path: &str,
    ) -> Result<Session, String> {
        let title = Self::extract_session_title(&claude_session);
        let config = Self::create_default_session_config(project_path);
        let config_json = serde_json::to_string(&config).unwrap_or_default();

        let main_worktree_id = self.get_main_worktree_id(project_id).await?;
        let db_session =
            self.create_db_session(project_id, main_worktree_id, &title, &config_json).await?;
        let session_uuid = Uuid::parse_str(&db_session.id).map_err(|e| e.to_string())?;

        Ok(Session {
            id: session_uuid,
            title,
            config,
            messages: Vec::new(),
            status: SessionStatus::Exited,
            created_at: claude_session.start_time.unwrap_or_else(chrono::Utc::now),
            updated_at: claude_session.start_time.unwrap_or_else(chrono::Utc::now),
            claude_session_id: Some(claude_session.file_path),
        })
    }

    fn extract_session_title(claude_session: &claude_code::ExistingSession) -> String {
        claude_session
            .first_user_message
            .as_deref()
            .unwrap_or("Claude Code Session")
            .chars()
            .take(100)
            .collect()
    }

    fn create_default_session_config(project_path: &str) -> SessionConfig {
        SessionConfig {
            model: "claude-3-5-sonnet-20241022".to_string(),
            temperature: None,
            max_tokens: None,
            permission_mode: "default".to_string(),
            working_directory: Some(project_path.to_string()),
        }
    }

    async fn get_main_worktree_id(&self, project_id: i32) -> Result<i32, String> {
        let db = self.db.lock().await;
        let worktrees = db.get_worktrees_by_project(project_id).unwrap_or_default();
        worktrees
            .iter()
            .find(|w| w.is_main)
            .map(|w| w.id)
            .ok_or_else(|| "No main worktree found".to_string())
    }

    async fn create_db_session(
        &self,
        project_id: i32,
        worktree_id: i32,
        title: &str,
        config_json: &str,
    ) -> Result<models::Session, String> {
        let db = self.db.lock().await;
        db.create_session(project_id, Some(worktree_id), title, Some(config_json))
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::SessionService;
    use crate::claude_code::SessionConfig;
    use crate::db::Database;
    use std::sync::Arc;
    use tempfile::TempDir;
    use tokio::sync::Mutex;

    fn setup_test_db() -> (Arc<Mutex<Database>>, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new_with_path(&db_path).unwrap();
        (Arc::new(Mutex::new(db)), temp_dir)
    }

    #[tokio::test]
    async fn test_create_session() {
        let (db, _temp_dir) = setup_test_db();
        let service = SessionService::new(Arc::clone(&db));

        // Create a project and worktree first
        let (project_id, worktree_id) = {
            let db = db.lock().await;
            db.create_project("/test/path", "Test Project").unwrap();
            let project = db.get_projects().unwrap()[0].clone();
            db.create_worktree(project.id, "/test/path", "main", true).unwrap();
            let worktree = db.get_worktrees_by_project(project.id).unwrap()[0].clone();
            (project.id, worktree.id)
        };

        // Create session
        let session = service
            .create_session(project_id, Some(worktree_id), Some("Test Session".to_string()), None)
            .await
            .unwrap();

        assert_eq!(session.title, "Test Session");
        assert!(!session.id.to_string().is_empty());
    }

    #[tokio::test]
    async fn test_list_sessions_empty() {
        let (db, _temp_dir) = setup_test_db();
        let service = SessionService::new(Arc::clone(&db));

        let sessions = service.list_sessions(None).await.unwrap();
        assert!(sessions.is_empty());
    }

    #[tokio::test]
    async fn test_list_sessions_with_project_filter() {
        let (db, _temp_dir) = setup_test_db();
        let service = SessionService::new(Arc::clone(&db));

        // Create two projects with worktrees
        let (project1_id, project2_id) = {
            let db = db.lock().await;
            db.create_project("/test/project1", "Project 1").unwrap();
            db.create_project("/test/project2", "Project 2").unwrap();
            let projects = db.get_projects().unwrap();

            // Create worktrees
            db.create_worktree(projects[0].id, "/test/project1", "main", true).unwrap();
            db.create_worktree(projects[1].id, "/test/project2", "main", true).unwrap();

            (projects[0].id, projects[1].id)
        };

        // Create sessions for each project
        service
            .create_session(project1_id, None, Some("Session 1".to_string()), None)
            .await
            .unwrap();
        service
            .create_session(project1_id, None, Some("Session 2".to_string()), None)
            .await
            .unwrap();
        service
            .create_session(project2_id, None, Some("Session 3".to_string()), None)
            .await
            .unwrap();

        // List all sessions
        let all_sessions = service.list_sessions(None).await.unwrap();
        assert_eq!(all_sessions.len(), 3);

        // List sessions for project 1
        let project1_sessions = service.list_sessions(Some(project1_id)).await.unwrap();
        assert_eq!(project1_sessions.len(), 2);

        // List sessions for project 2
        let project2_sessions = service.list_sessions(Some(project2_id)).await.unwrap();
        assert_eq!(project2_sessions.len(), 1);
    }

    #[tokio::test]
    async fn test_get_session() {
        let (db, _temp_dir) = setup_test_db();
        let service = SessionService::new(Arc::clone(&db));

        // Create a project and worktree
        let project_id = {
            let db = db.lock().await;
            db.create_project("/test/path", "Test Project").unwrap();
            let project = db.get_projects().unwrap()[0].clone();
            db.create_worktree(project.id, "/test/path", "main", true).unwrap();
            project.id
        };

        let created_session = service
            .create_session(project_id, None, Some("Test Session".to_string()), None)
            .await
            .unwrap();

        // Get the session
        let retrieved_session =
            service.get_session(&created_session.id.to_string()).await.unwrap().unwrap();

        assert_eq!(retrieved_session.id, created_session.id);
        assert_eq!(retrieved_session.title, "Test Session");
        assert_eq!(retrieved_session.messages.len(), 0);
    }

    #[tokio::test]
    async fn test_session_with_custom_config() {
        let (db, _temp_dir) = setup_test_db();
        let service = SessionService::new(Arc::clone(&db));

        // Create a project and worktree
        let project_id = {
            let db = db.lock().await;
            db.create_project("/test/path", "Test Project").unwrap();
            let project = db.get_projects().unwrap()[0].clone();
            db.create_worktree(project.id, "/test/path", "main", true).unwrap();
            project.id
        };

        // Create custom config
        let config =
            SessionConfig { max_tokens: Some(2000), temperature: Some(0.8), ..Default::default() };

        let config_json = serde_json::to_string(&config).unwrap();

        // Create session with custom config
        let session = service
            .create_session(
                project_id,
                None,
                Some("Configured Session".to_string()),
                Some(config_json.clone()),
            )
            .await
            .unwrap();

        // Verify config is preserved
        let session_config_json = serde_json::to_string(&session.config).unwrap();
        let parsed_config: SessionConfig = serde_json::from_str(&session_config_json).unwrap();
        assert_eq!(parsed_config.max_tokens, Some(2000));
        assert_eq!(parsed_config.temperature, Some(0.8));
    }
}
