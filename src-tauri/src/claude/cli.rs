use super::message::{Message, MessageContent, MessageRole, MessageType};
use anyhow::Result;
use serde::Deserialize;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Claude CLI streaming response types
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum StreamingResponse {
    MessageStart { message: StreamingMessage },
    ContentBlockStart { index: usize, content_block: ContentBlock },
    ContentBlockDelta { index: usize, delta: ContentDelta },
    ContentBlockStop { index: usize },
    MessageDelta { delta: MessageDelta, usage: Usage },
    MessageStop {},
    Error { error: ErrorDetail },
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct StreamingMessage {
    pub id: String,
    pub role: String,
    pub content: Vec<serde_json::Value>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum ContentBlock {
    Text { text: String },
    ToolUse { id: String, name: String, input: serde_json::Value },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ContentDelta {
    TextDelta { text: String },
    InputJsonDelta { partial_json: String },
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct MessageDelta {
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct ErrorDetail {
    pub r#type: String,
    pub message: String,
}

pub struct ClaudeCLI {
    process: Option<Child>,
    tx: Option<mpsc::UnboundedSender<Message>>,
}

impl ClaudeCLI {
    pub fn new() -> Self {
        Self { process: None, tx: None }
    }

    /// Start Claude CLI process with streaming output
    #[allow(clippy::too_many_lines)]
    pub fn start(
        &mut self,
        prompt: &str,
        permission_mode: &str,
        working_dir: Option<String>,
    ) -> Result<mpsc::UnboundedReceiver<Message>> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.tx = Some(tx.clone());

        let mut cmd = Command::new("claude");
        cmd.arg("--output-format")
            .arg("stream-json")
            .arg("--verbose")
            .arg("--permission-mode")
            .arg(permission_mode)
            .arg("-p")
            .arg(prompt)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        info!("Starting Claude CLI with permission mode: {}", permission_mode);
        let mut child = cmd.spawn()?;

        let stdout = child.stdout.take().expect("Failed to get stdout");
        let stderr = child.stderr.take().expect("Failed to get stderr");

        // Handle stdout (streaming JSON)
        let tx_stdout = tx.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            let mut current_text = String::new();
            let mut current_tool_name: Option<String> = None;
            let mut current_tool_input = String::new();

            while let Ok(Some(line)) = lines.next_line().await {
                if line.trim().is_empty() {
                    continue;
                }

                debug!("Claude CLI output: {}", line);

                match serde_json::from_str::<StreamingResponse>(&line) {
                    Ok(response) => match response {
                        StreamingResponse::ContentBlockStart { content_block, .. } => {
                            match content_block {
                                ContentBlock::Text { .. } => {
                                    current_text.clear();
                                }
                                ContentBlock::ToolUse { name, .. } => {
                                    current_tool_name = Some(name);
                                    current_tool_input.clear();
                                }
                            }
                        }
                        StreamingResponse::ContentBlockDelta { delta, .. } => match delta {
                            ContentDelta::TextDelta { text } => {
                                current_text.push_str(&text);
                                // Send incremental text update
                                let _ = tx_stdout.send(Message::new_text(
                                    MessageRole::Assistant,
                                    current_text.clone(),
                                ));
                            }
                            ContentDelta::InputJsonDelta { partial_json } => {
                                current_tool_input.push_str(&partial_json);
                            }
                        },
                        StreamingResponse::ContentBlockStop { .. } => {
                            if let Some(tool_name) = current_tool_name.take() {
                                // Parse and send tool use message
                                if let Ok(input) = serde_json::from_str(&current_tool_input) {
                                    let message = Message {
                                        id: uuid::Uuid::new_v4(),
                                        role: MessageRole::Assistant,
                                        message_type: MessageType::ToolUse,
                                        content: MessageContent {
                                            text: None,
                                            tool_name: Some(tool_name),
                                            tool_input: Some(input),
                                            tool_result: None,
                                            error: None,
                                        },
                                        timestamp: chrono::Utc::now(),
                                    };
                                    let _ = tx_stdout.send(message);
                                }
                                current_tool_input.clear();
                            }
                        }
                        StreamingResponse::Error { error } => {
                            let _ = tx_stdout.send(Message::new_error(error.message));
                        }
                        _ => {}
                    },
                    Err(e) => {
                        warn!("Failed to parse Claude response: {} - Line: {}", e, line);
                    }
                }
            }
        });

        // Handle stderr
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if !line.trim().is_empty() {
                    error!("Claude CLI stderr: {}", line);
                    let _ = tx.send(Message::new_error(format!("CLI Error: {line}")));
                }
            }
        });

        self.process = Some(child);
        Ok(rx)
    }

    /// Stop the Claude CLI process
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.take() {
            info!("Stopping Claude CLI process");
            process.kill().await?;
        }
        Ok(())
    }
}

impl Drop for ClaudeCLI {
    fn drop(&mut self) {
        if let Some(mut process) = self.process.take() {
            // Try to kill the process
            let _ = process.start_kill();
        }
    }
}
