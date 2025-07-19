use super::message::Message;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SessionStatus {
    /// Process is currently running
    Running,
    /// Process exited normally (exit code 0)
    Exited,
    /// Process exited with an error (non-zero exit code)
    Failed {
        #[serde(rename = "exitCode")]
        exit_code: i32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionConfig {
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub permission_mode: String,
    pub working_directory: Option<String>,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            model: "claude-3-5-sonnet-20241022".to_string(),
            temperature: None,
            max_tokens: None,
            permission_mode: "default".to_string(),
            working_directory: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: Uuid,
    pub title: String,
    pub config: SessionConfig,
    pub messages: Vec<Message>,
    pub status: SessionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Claude CLI's internal session ID for conversation continuity
    pub claude_session_id: Option<String>,
}

impl Session {
    pub fn new(title: String, config: SessionConfig) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            config,
            messages: Vec::new(),
            status: SessionStatus::Exited,
            created_at: now,
            updated_at: now,
            claude_session_id: None,
        }
    }
}
