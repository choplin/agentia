use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub path: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settings: Option<String>, // JSON string for flexibility
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worktree {
    pub id: i32,
    pub project_id: i32,
    pub path: String,
    pub name: String,
    pub is_main: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub project_id: i32,
    pub worktree_id: i32,
    pub title: String,
    pub default_config: Option<String>, // JSON string for SessionConfig
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCliSession {
    pub id: i32,
    pub session_id: String,
    pub claude_session_id: String,
    pub file_path: String,
    pub config: String, // JSON string for actual config used
    pub parent_cli_session_id: Option<i32>,
    pub process_pid: Option<i32>,
    pub exit_code: Option<i32>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveCliSession {
    pub session_id: String,
    pub cli_session_id: i32,
    pub started_at: DateTime<Utc>,
}
