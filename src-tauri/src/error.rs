use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    Database(String),
    AlreadyExists(String),
    InvalidInput(String),
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Database(msg) => write!(f, "Database error: {msg}"),
            AppError::AlreadyExists(msg) => write!(f, "Already exists: {msg}"),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            AppError::Internal(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        AppError::Database(error.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Internal(error.to_string())
    }
}

// AppError already implements Serialize, so Tauri will automatically convert it

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let db_error = AppError::Database("connection failed".to_string());
        assert_eq!(db_error.to_string(), "Database error: connection failed");

        let exists_error = AppError::AlreadyExists("project at /path".to_string());
        assert_eq!(exists_error.to_string(), "Already exists: project at /path");

        let input_error = AppError::InvalidInput("empty name".to_string());
        assert_eq!(input_error.to_string(), "Invalid input: empty name");

        let internal_error = AppError::Internal("unexpected".to_string());
        assert_eq!(internal_error.to_string(), "Internal error: unexpected");
    }

    #[test]
    fn test_from_rusqlite_error() {
        use rusqlite::Connection;

        // Create an actual rusqlite error
        let conn = Connection::open_in_memory().unwrap();
        let result: Result<i32, rusqlite::Error> = conn
            .prepare("SELECT * FROM non_existent_table")
            .and_then(|mut stmt| stmt.query_row([], |row| row.get(0)));

        let error = result.unwrap_err();
        let app_error: AppError = error.into();

        match app_error {
            AppError::Database(msg) => assert!(msg.contains("no such table")),
            _ => panic!("Expected Database error"),
        }
    }

    #[test]
    fn test_from_io_error() {
        use std::io;

        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let app_error: AppError = io_error.into();

        match app_error {
            AppError::Internal(msg) => assert_eq!(msg, "file not found"),
            _ => panic!("Expected Internal error"),
        }
    }

    #[test]
    fn test_error_serialization() {
        let error = AppError::AlreadyExists("test".to_string());
        let json = serde_json::to_string(&error).unwrap();
        let expected = r#"{"type":"AlreadyExists","message":"test"}"#;
        assert_eq!(json, expected);

        let deserialized: AppError = serde_json::from_str(&json).unwrap();
        match deserialized {
            AppError::AlreadyExists(msg) => assert_eq!(msg, "test"),
            _ => panic!("Wrong error type after deserialization"),
        }
    }
}
