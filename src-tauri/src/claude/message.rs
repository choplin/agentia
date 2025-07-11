use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    Text,
    ToolUse,
    ToolResult,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageContent {
    pub text: Option<String>,
    pub tool_name: Option<String>,
    pub tool_input: Option<serde_json::Value>,
    pub tool_result: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: Uuid,
    pub role: MessageRole,
    #[serde(rename = "type")]
    #[allow(clippy::struct_field_names)]
    pub message_type: MessageType,
    pub content: MessageContent,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn new_text(role: MessageRole, text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role,
            message_type: MessageType::Text,
            content: MessageContent {
                text: Some(text),
                tool_name: None,
                tool_input: None,
                tool_result: None,
                error: None,
            },
            timestamp: Utc::now(),
        }
    }

    pub fn new_error(error: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::Assistant,
            message_type: MessageType::Error,
            content: MessageContent {
                text: None,
                tool_name: None,
                tool_input: None,
                tool_result: None,
                error: Some(error),
            },
            timestamp: Utc::now(),
        }
    }
}
