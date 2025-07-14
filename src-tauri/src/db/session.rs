use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

use super::models::Session;

/// Create a new session
pub fn create(
    conn: &Connection,
    project_id: &str,
    claude_session_id: Option<&str>,
    title: &str,
    config: Option<&str>,
) -> Result<Session> {
    let session = Session {
        id: Uuid::new_v4().to_string(),
        project_id: project_id.to_string(),
        claude_session_id: claude_session_id.map(ToString::to_string),
        title: title.to_string(),
        config: config.map(ToString::to_string),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    conn.execute(
        "INSERT INTO sessions (id, project_id, claude_session_id, title, config, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            session.id,
            session.project_id,
            session.claude_session_id,
            session.title,
            session.config,
            session.created_at,
            session.updated_at,
        ],
    )?;

    Ok(session)
}

/// Update Claude session ID
pub fn update_claude_session_id(
    conn: &Connection,
    session_id: &str,
    claude_session_id: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE sessions SET claude_session_id = ?1, updated_at = ?2 WHERE id = ?3",
        params![claude_session_id, chrono::Utc::now(), session_id],
    )?;
    Ok(())
}

/// Find sessions by project
pub fn find_by_project(conn: &Connection, project_id: &str) -> Result<Vec<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, claude_session_id, title, config, created_at, updated_at
         FROM sessions WHERE project_id = ?1 ORDER BY updated_at DESC",
    )?;

    let sessions = stmt
        .query_map([project_id], |row| {
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                claude_session_id: row.get(2)?,
                title: row.get(3)?,
                config: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(sessions)
}

/// Find session by ID
pub fn find_by_id(conn: &Connection, session_id: &str) -> Result<Option<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, claude_session_id, title, config, created_at, updated_at FROM sessions WHERE id = ?1"
    )?;

    let session = stmt
        .query_row([session_id], |row| {
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                claude_session_id: row.get(2)?,
                title: row.get(3)?,
                config: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .optional()?;

    Ok(session)
}
