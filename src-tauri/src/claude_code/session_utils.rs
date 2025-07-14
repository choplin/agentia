use std::path::Path;

/// Extract UUID from Claude Code session file path
///
/// Claude Code session files are named like:
/// `/home/user/.claude/projects/-home-user-project/011ad736-7a96-4edf-bf3a-1ebcc0d91cdb.jsonl`
///
/// This function extracts the UUID part (without .jsonl extension)
pub fn extract_session_uuid(file_path: &str) -> Option<String> {
    Path::new(file_path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(std::string::ToString::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_session_uuid() {
        let path = "/home/user/.claude/projects/-home-user-project/011ad736-7a96-4edf-bf3a-1ebcc0d91cdb.jsonl";
        assert_eq!(
            extract_session_uuid(path),
            Some("011ad736-7a96-4edf-bf3a-1ebcc0d91cdb".to_string())
        );
    }

    #[test]
    fn test_extract_session_uuid_no_extension() {
        let path = "/path/to/011ad736-7a96-4edf-bf3a-1ebcc0d91cdb";
        assert_eq!(
            extract_session_uuid(path),
            Some("011ad736-7a96-4edf-bf3a-1ebcc0d91cdb".to_string())
        );
    }

    #[test]
    fn test_extract_session_uuid_invalid_path() {
        assert_eq!(extract_session_uuid(""), None);
    }
}
