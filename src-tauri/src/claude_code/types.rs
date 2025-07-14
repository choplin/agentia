use crate::claude_code::{Message, MessageContent, MessageRole, MessageType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common message format for Claude Code logs and streaming responses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaudeMessage {
    /// Format used in Claude Code session logs
    SessionLog(Box<SessionLogEntry>),
    /// Format used in Claude CLI streaming responses
    StreamResponse(StreamResponseEntry),
}

/// Claude Code session log entry format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLogEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub timestamp: Option<DateTime<Utc>>,
    pub message: Option<MessagePayload>,
    pub uuid: Option<Uuid>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    #[serde(rename = "parentUuid")]
    pub parent_uuid: Option<Uuid>,
    #[serde(rename = "isSidechain")]
    pub is_sidechain: Option<bool>,
    #[serde(rename = "userType")]
    pub user_type: Option<String>,
    pub cwd: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    #[serde(rename = "isMeta")]
    pub is_meta: Option<bool>,

    // For summary entries
    pub summary: Option<String>,
    #[serde(rename = "leafUuid")]
    pub leaf_uuid: Option<String>,

    // For tool results
    #[serde(rename = "toolUseResult")]
    pub tool_use_result: Option<Vec<ContentItem>>,
}

/// Claude CLI streaming response entry format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "claude_completion")]
#[serde(rename_all = "snake_case")]
pub enum StreamResponseEntry {
    Completion(CompletionData),
    ContentBlockStart { index: usize, content_block: ContentBlock },
    ContentBlockDelta { index: usize, delta: ContentDelta },
    ContentBlockStop { index: usize },
    MessageStart { message: MessageStart },
    MessageDelta { delta: MessageDelta, usage: Usage },
    MessageStop,
    Ping,
    Error { error: ErrorDetail },
}

/// Message payload that can contain various content formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    pub role: String,
    #[serde(flatten)]
    pub content_wrapper: ContentWrapper,

    // Assistant-specific fields
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub msg_type: Option<String>,
    pub model: Option<String>,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Option<Usage>,
}

/// Wrapper for different content formats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentWrapper {
    /// Simple string content
    Text { content: String },
    /// Array of content items
    Array { content: Vec<ContentItem> },
}

/// Content item that can be text, tool use, or tool result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ContentItem {
    Text {
        text: String,
    },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        #[serde(rename = "tool_use_id")]
        tool_use_id: String,
        content: Option<Vec<ContentItem>>,
        #[serde(default)]
        is_error: bool,
    },
}

/// Streaming response structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionData {
    pub completion: String,
    pub stop_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ContentBlock {
    Text { text: String },
    ToolUse { id: String, name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ContentDelta {
    TextDelta { text: String },
    InputJsonDelta { partial_json: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStart {
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub role: String,
    pub model: String,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDelta {
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "input_tokens")]
    pub input: u32,
    #[serde(rename = "output_tokens")]
    pub output: u32,
    #[serde(default)]
    #[serde(rename = "cache_creation_input_tokens")]
    pub cache_creation_input: Option<u32>,
    #[serde(default)]
    #[serde(rename = "cache_read_input_tokens")]
    pub cache_read_input: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

/// Convert a Claude message (either format) to our internal Message format
#[allow(dead_code)]
impl ClaudeMessage {
    pub fn to_internal_message(&self) -> Option<Message> {
        match self {
            ClaudeMessage::SessionLog(entry) => entry.to_internal_message(),
            ClaudeMessage::StreamResponse(_) => None, // Streaming responses are handled differently
        }
    }
}

impl SessionLogEntry {
    pub fn to_internal_message(&self) -> Option<Message> {
        // Skip summary entries and meta entries
        if self.entry_type == "summary" || self.is_meta == Some(true) {
            return None;
        }

        let message = self.message.as_ref()?;
        let timestamp = self.timestamp.unwrap_or_else(Utc::now);
        let id = self.uuid.unwrap_or_else(Uuid::new_v4);

        match self.entry_type.as_str() {
            "user" => {
                let text = match &message.content_wrapper {
                    ContentWrapper::Text { content } => content.clone(),
                    ContentWrapper::Array { content } => content
                        .iter()
                        .filter_map(|item| match item {
                            ContentItem::Text { text } => Some(text.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("\n"),
                };

                Some(Message {
                    id,
                    role: MessageRole::User,
                    message_type: MessageType::Text,
                    content: MessageContent {
                        text: Some(text),
                        tool_name: None,
                        tool_input: None,
                        tool_result: None,
                        error: None,
                    },
                    timestamp,
                })
            }
            "assistant" => {
                // Process assistant messages - could be text or tool use
                if let ContentWrapper::Array { content } = &message.content_wrapper {
                    // For now, just handle the first content item
                    if let Some(first_item) = content.first() {
                        match first_item {
                            ContentItem::Text { text } => {
                                return Some(Message {
                                    id,
                                    role: MessageRole::Assistant,
                                    message_type: MessageType::Text,
                                    content: MessageContent {
                                        text: Some(text.clone()),
                                        tool_name: None,
                                        tool_input: None,
                                        tool_result: None,
                                        error: None,
                                    },
                                    timestamp,
                                });
                            }
                            ContentItem::ToolUse { name, input, .. } => {
                                return Some(Message {
                                    id,
                                    role: MessageRole::Assistant,
                                    message_type: MessageType::ToolUse,
                                    content: MessageContent {
                                        text: None,
                                        tool_name: Some(name.clone()),
                                        tool_input: Some(input.clone()),
                                        tool_result: None,
                                        error: None,
                                    },
                                    timestamp,
                                });
                            }
                            ContentItem::ToolResult { .. } => {}
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}
