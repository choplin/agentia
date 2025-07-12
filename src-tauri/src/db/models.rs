use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub path: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settings: Option<String>, // JSON string for flexibility
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub project_id: String,
    pub claude_session_id: Option<String>,
    pub title: String,
    pub config: Option<String>, // JSON string for SessionConfig
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
