use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistingSession {
    pub session_id: String,
    pub file_path: String,
    pub project_path: String,
    pub start_time: Option<DateTime<Utc>>,
    pub message_count: usize,
    pub first_user_message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClaudeCodeEntry {
    #[serde(rename = "type")]
    entry_type: String,
    timestamp: Option<String>,
    #[serde(rename = "sessionId")]
    session_id: Option<String>,
    message: Option<serde_json::Value>,
    #[allow(dead_code)]
    summary: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "leafUuid")]
    leaf_uuid: Option<String>,
}

/// Decode Claude Code project directory name to original path
#[allow(dead_code)]
fn decode_project_path(encoded_name: &str) -> String {
    // Remove leading dash and replace remaining dashes with slashes
    encoded_name.strip_prefix('-').unwrap_or(encoded_name).replace('-', "/")
}

/// List all existing Claude Code sessions for a given project path
pub fn list_existing_sessions(project_path: &str) -> Result<Vec<ExistingSession>, String> {
    // Encode project path for Claude Code directory name
    let encoded_name = project_path.replace(['/', '.'], "-");
    let claude_projects_dir = dirs::home_dir()
        .ok_or("Failed to get home directory")?
        .join(".claude")
        .join("projects")
        .join(&encoded_name);

    if !claude_projects_dir.exists() {
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();

    // Read all .jsonl files in the directory
    let entries = fs::read_dir(&claude_projects_dir)
        .map_err(|e| format!("Failed to read directory: {e}"))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {e}"))?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
            if let Ok(session) = read_session_summary(&path, project_path) {
                sessions.push(session);
            }
        }
    }

    // Sort by start time (newest first)
    sessions.sort_by(|a, b| b.start_time.cmp(&a.start_time));

    Ok(sessions)
}

/// Read summary information from a Claude Code session file
fn read_session_summary(file_path: &Path, project_path: &str) -> Result<ExistingSession, String> {
    let file = fs::File::open(file_path).map_err(|e| format!("Failed to open file: {e}"))?;

    let mut session_id = None;
    let mut start_time = None;
    let mut first_user_message = None;
    let mut message_count = 0;
    let mut is_summary_only = true;

    // Read the file twice: once for initial analysis, once for counting
    let reader = BufReader::new(&file);
    let mut lines_read = 0;

    // Read first few lines to determine file type and extract info
    for (i, line) in reader.lines().enumerate() {
        lines_read = i + 1;
        if i > 20 && session_id.is_some() {
            break;
        }

        let line = line.map_err(|e| format!("Failed to read line: {e}"))?;
        if line.trim().is_empty() {
            continue;
        }

        let entry: ClaudeCodeEntry =
            serde_json::from_str(&line).map_err(|e| format!("Failed to parse JSON: {e}"))?;

        // Check if this is a conversation entry (has timestamp)
        if entry.timestamp.is_some() {
            is_summary_only = false;
        }

        // Skip summary-only files
        if i > 10 && is_summary_only {
            return Err("Summary-only file".to_string());
        }

        // Extract session ID
        if session_id.is_none() && entry.session_id.is_some() {
            session_id = entry.session_id;
        }

        // Extract start time from first timestamped entry
        if start_time.is_none() && entry.timestamp.is_some() {
            start_time = entry
                .timestamp
                .as_ref()
                .and_then(|ts| DateTime::parse_from_rfc3339(ts).ok())
                .map(|dt| dt.with_timezone(&Utc));
        }

        // Extract first user message
        if first_user_message.is_none() && entry.entry_type == "user" {
            if let Some(msg) = entry.message {
                first_user_message = extract_user_message_text(&msg);
            }
        }
    }

    // Count total lines if we need more info
    if session_id.is_some() && !is_summary_only {
        // Reopen file to count all lines
        let file2 =
            fs::File::open(file_path).map_err(|e| format!("Failed to reopen file: {e}"))?;
        let reader2 = BufReader::new(file2);
        message_count = lines_read + reader2.lines().count();
    }

    // Use filename as session ID if not found in content
    let session_id = session_id.unwrap_or_else(|| {
        file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_string()
    });

    Ok(ExistingSession {
        session_id,
        file_path: file_path.to_string_lossy().to_string(),
        project_path: project_path.to_string(),
        start_time,
        message_count,
        first_user_message,
    })
}

/// Extract text from user message JSON
fn extract_user_message_text(message: &serde_json::Value) -> Option<String> {
    // Handle string content
    if let Some(content) = message.get("content") {
        if let Some(text) = content.as_str() {
            return Some(text.to_string());
        }

        // Handle array content format
        if let Some(arr) = content.as_array() {
            for item in arr {
                if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                    return Some(text.to_string());
                }
            }
        }
    }

    None
}

/// Read and parse a full Claude Code session file
pub fn read_session_file(file_path: &str) -> Result<Vec<super::types::SessionLogEntry>, String> {
    let file = fs::File::open(file_path).map_err(|e| format!("Failed to open file: {e}"))?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {e}"))?;
        if line.trim().is_empty() {
            continue;
        }

        let entry: super::types::SessionLogEntry =
            serde_json::from_str(&line).map_err(|e| format!("Failed to parse JSON: {e}"))?;
        entries.push(entry);
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_project_path() {
        assert_eq!(
            decode_project_path("-Users-aki-workspace-agentia"),
            "/Users/aki/workspace/agentia"
        );
    }
}
