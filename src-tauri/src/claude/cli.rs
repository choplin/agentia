use super::message::{Message, MessageContent, MessageRole, MessageType};
use anyhow::Result;
use serde::Deserialize;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Claude CLI response types (both streaming and non-streaming)
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum ClaudeResponse {
    // Streaming response types
    MessageStart { message: StreamingMessage },
    ContentBlockStart { index: usize, content_block: ContentBlock },
    ContentBlockDelta { index: usize, delta: ContentDelta },
    ContentBlockStop { index: usize },
    MessageDelta { delta: MessageDelta, usage: Usage },
    MessageStop {},
    Error { error: ErrorDetail },
    // Non-streaming response types
    System { subtype: String, session_id: Option<String> },
    Assistant { message: AssistantMessage },
    Result { subtype: String, result: Option<String>, is_error: bool },
    User { message: UserMessage },
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
#[allow(dead_code)]
pub struct AssistantMessage {
    pub id: String,
    pub role: String,
    pub content: Vec<AssistantContent>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum AssistantContent {
    Text {
        text: String,
    },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
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

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct UserMessage {
    pub role: String,
    pub content: Vec<UserContent>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum UserContent {
    Text {
        text: String,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        content: String,
        #[serde(default)]
        is_error: bool,
        tool_use_id: String,
    },
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum OutputFormat {
    Text,
    Json,
    StreamJson,
}

impl OutputFormat {
    fn as_str(self) -> &'static str {
        match self {
            OutputFormat::Text => "text",
            OutputFormat::Json => "json",
            OutputFormat::StreamJson => "stream-json",
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum InputFormat {
    Text,
    StreamJson,
}

impl InputFormat {
    fn as_str(self) -> &'static str {
        match self {
            InputFormat::Text => "text",
            InputFormat::StreamJson => "stream-json",
        }
    }
}

/// Builder pattern for Claude CLI commands
#[derive(Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct ClaudeCommandBuilder {
    // Required
    prompt: Option<String>,

    // Modes
    print_mode: bool,
    continue_mode: bool,
    resume_session_id: Option<String>,

    // Formats
    output_format: Option<OutputFormat>,
    input_format: Option<InputFormat>,

    // Options
    model: Option<String>,
    fallback_model: Option<String>,
    max_turns: Option<u32>,
    verbose: bool,
    debug: bool,

    // Permissions
    permission_mode: Option<String>,
    dangerously_skip_permissions: bool,
    allowed_tools: Vec<String>,
    disallowed_tools: Vec<String>,

    // Directories
    working_dir: Option<String>,
    add_dirs: Vec<String>,

    // MCP
    mcp_config: Option<String>,
    strict_mcp_config: bool,

    // IDE
    ide: bool,
}


#[allow(dead_code)]
impl ClaudeCommandBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn print_mode(mut self) -> Self {
        self.print_mode = true;
        self
    }

    pub fn continue_conversation(mut self) -> Self {
        self.continue_mode = true;
        self
    }

    pub fn resume(mut self, session_id: impl Into<String>) -> Self {
        self.resume_session_id = Some(session_id.into());
        self
    }

    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = Some(format);
        self
    }

    pub fn input_format(mut self, format: InputFormat) -> Self {
        self.input_format = Some(format);
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn fallback_model(mut self, model: impl Into<String>) -> Self {
        self.fallback_model = Some(model.into());
        self
    }

    pub fn max_turns(mut self, turns: u32) -> Self {
        self.max_turns = Some(turns);
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn permission_mode(mut self, mode: impl Into<String>) -> Self {
        self.permission_mode = Some(mode.into());
        self
    }

    pub fn dangerously_skip_permissions(mut self) -> Self {
        self.dangerously_skip_permissions = true;
        self
    }

    pub fn allow_tools(mut self, tools: Vec<String>) -> Self {
        self.allowed_tools = tools;
        self
    }

    pub fn disallow_tools(mut self, tools: Vec<String>) -> Self {
        self.disallowed_tools = tools;
        self
    }

    pub fn working_dir(mut self, dir: impl Into<String>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    pub fn add_directories(mut self, dirs: Vec<String>) -> Self {
        self.add_dirs = dirs;
        self
    }

    pub fn mcp_config(mut self, config: impl Into<String>) -> Self {
        self.mcp_config = Some(config.into());
        self
    }

    pub fn strict_mcp_config(mut self) -> Self {
        self.strict_mcp_config = true;
        self
    }

    pub fn ide(mut self) -> Self {
        self.ide = true;
        self
    }

    /// Build the command
    pub fn build(self) -> Command {
        let mut cmd = Command::new("claude");

        // Mode flags
        if self.print_mode {
            cmd.arg("--print");
        }

        if self.continue_mode {
            cmd.arg("--continue");
        }

        if let Some(session_id) = self.resume_session_id {
            cmd.arg("--resume").arg(session_id);
        }

        // Format flags
        if let Some(format) = self.output_format {
            cmd.arg("--output-format").arg(format.as_str());
        }

        if let Some(format) = self.input_format {
            cmd.arg("--input-format").arg(format.as_str());
        }

        // Model flags
        if let Some(model) = self.model {
            cmd.arg("--model").arg(model);
        }

        if let Some(model) = self.fallback_model {
            cmd.arg("--fallback-model").arg(model);
        }

        // Options
        if let Some(turns) = self.max_turns {
            cmd.arg("--max-turns").arg(turns.to_string());
        }

        if self.verbose {
            cmd.arg("--verbose");
        }

        if self.debug {
            cmd.arg("--debug");
        }

        // Permissions
        if let Some(mode) = self.permission_mode {
            cmd.arg("--permission-mode").arg(mode);
        }

        if self.dangerously_skip_permissions {
            cmd.arg("--dangerously-skip-permissions");
        }

        if !self.allowed_tools.is_empty() {
            cmd.arg("--allowedTools").arg(self.allowed_tools.join(" "));
        }

        if !self.disallowed_tools.is_empty() {
            cmd.arg("--disallowedTools").arg(self.disallowed_tools.join(" "));
        }

        // Directories
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }

        if !self.add_dirs.is_empty() {
            cmd.arg("--add-dir").arg(self.add_dirs.join(" "));
        }

        // MCP
        if let Some(config) = self.mcp_config {
            cmd.arg("--mcp-config").arg(config);
        }

        if self.strict_mcp_config {
            cmd.arg("--strict-mcp-config");
        }

        // IDE
        if self.ide {
            cmd.arg("--ide");
        }

        // Prompt must be last
        if let Some(prompt) = self.prompt {
            cmd.arg(prompt);
        }

        // Set up pipes
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        cmd
    }
}

pub struct ClaudeCLI {
    // Claude CLIから返されたセッションIDを保持
    claude_session_id: Option<String>,
}

impl ClaudeCLI {
    pub fn new() -> Self {
        Self { claude_session_id: None }
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.claude_session_id = Some(session_id);
    }

    pub fn get_session_id(&self) -> Option<&String> {
        self.claude_session_id.as_ref()
    }

    /// Send a message using Claude CLI with builder pattern
    #[allow(clippy::too_many_lines)]
    pub fn send_message(
        &mut self,
        builder: ClaudeCommandBuilder,
    ) -> Result<mpsc::UnboundedReceiver<Message>> {
        let (tx, rx) = mpsc::unbounded_channel();
        let (session_tx, mut session_rx) = mpsc::unbounded_channel::<String>();

        let mut cmd = builder.build();

        info!("Starting Claude CLI process");
        let mut child = cmd.spawn()?;

        let stdout = child.stdout.take().expect("Failed to get stdout");
        let stderr = child.stderr.take().expect("Failed to get stderr");

        // Handle stdout (streaming JSON)
        let tx_stdout = tx.clone();
        let claude_session_id = self.claude_session_id.clone();

        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            let mut current_text = String::new();
            let mut current_tool_name: Option<String> = None;
            let mut current_tool_input = String::new();
            let mut _session_id_holder: Option<String> = claude_session_id;

            while let Ok(Some(line)) = lines.next_line().await {
                if line.trim().is_empty() {
                    continue;
                }

                debug!("Claude CLI output: {}", line);

                match serde_json::from_str::<ClaudeResponse>(&line) {
                    Ok(response) => match response {
                        ClaudeResponse::System { subtype, session_id } => {
                            info!("Claude CLI system message: {}", subtype);

                            // Save session ID if this is init response
                            if subtype == "init" {
                                if let Some(sid) = session_id {
                                    debug!("Received Claude session ID: {}", sid);
                                    _session_id_holder = Some(sid.clone());
                                    let _ = session_tx.send(sid);
                                }
                            }
                        }
                        ClaudeResponse::Assistant { message } => {
                            // Handle complete assistant message
                            for content in &message.content {
                                match content {
                                    AssistantContent::Text { text } => {
                                        let msg =
                                            Message::new_text(MessageRole::Assistant, text.clone());
                                        let _ = tx_stdout.send(msg);
                                    }
                                    AssistantContent::ToolUse { name, input, .. } => {
                                        let msg = Message {
                                            id: uuid::Uuid::new_v4(),
                                            role: MessageRole::Assistant,
                                            message_type: MessageType::ToolUse,
                                            content: MessageContent {
                                                text: None,
                                                tool_name: Some(name.clone()),
                                                tool_input: Some(input.clone()),
                                                tool_result: None,
                                                error: None,
                                            },
                                            timestamp: chrono::Utc::now(),
                                        };
                                        let _ = tx_stdout.send(msg);
                                    }
                                }
                            }
                        }
                        ClaudeResponse::Result { subtype, result, is_error } => {
                            if is_error {
                                if let Some(error_msg) = result {
                                    let _ = tx_stdout.send(Message::new_error(error_msg));
                                }
                            } else {
                                debug!(
                                    "Claude CLI result: subtype={}, result={:?}",
                                    subtype, result
                                );
                            }
                            // Result メッセージを受信したら処理を終了
                            break;
                        }
                        ClaudeResponse::ContentBlockStart { content_block, .. } => {
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
                        ClaudeResponse::ContentBlockDelta { delta, .. } => match delta {
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
                        ClaudeResponse::ContentBlockStop { .. } => {
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
                        ClaudeResponse::Error { error } => {
                            let _ = tx_stdout.send(Message::new_error(error.message));
                        }
                        ClaudeResponse::User { message } => {
                            // Handle user messages (permission requests)
                            for content in &message.content {
                                match content {
                                    UserContent::Text { text } => {
                                        let msg =
                                            Message::new_text(MessageRole::User, text.clone());
                                        let _ = tx_stdout.send(msg);
                                    }
                                    UserContent::ToolResult { content, is_error, .. } => {
                                        if *is_error {
                                            let _ =
                                                tx_stdout.send(Message::new_error(content.clone()));
                                        } else {
                                            let msg = Message::new_text(
                                                MessageRole::User,
                                                content.clone(),
                                            );
                                            let _ = tx_stdout.send(msg);
                                        }
                                    }
                                }
                            }
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

        // Wait for the process to complete
        tokio::spawn(async move {
            let _ = child.wait().await;
        });

        // Check for session ID from the spawned task
        if let Ok(session_id) = session_rx.try_recv() {
            self.set_session_id(session_id);
        }

        Ok(rx)
    }
}
