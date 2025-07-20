use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

use super::models::Session;

/// Create a new session
pub fn create(
    conn: &Connection,
    project_id: i32,
    worktree_id: i32,
    title: &str,
    default_config: Option<&str>,
) -> Result<Session> {
    let now = Utc::now();
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO sessions (id, project_id, worktree_id, title, default_config, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            id,
            project_id,
            worktree_id,
            title,
            default_config,
            now.to_rfc3339(),
            now.to_rfc3339(),
        ],
    )?;

    Ok(Session {
        id,
        project_id,
        worktree_id,
        title: title.to_string(),
        default_config: default_config.map(ToString::to_string),
        created_at: now,
        updated_at: now,
    })
}

/// Update session's `updated_at` timestamp
pub fn touch(conn: &Connection, session_id: &str) -> Result<()> {
    let now = Utc::now();
    conn.execute(
        "UPDATE sessions SET updated_at = ?1 WHERE id = ?2",
        params![now.to_rfc3339(), session_id],
    )?;
    Ok(())
}

/// Find all sessions
pub fn find_all(conn: &Connection) -> Result<Vec<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, worktree_id, title, default_config, created_at, updated_at
         FROM sessions ORDER BY updated_at DESC",
    )?;

    let sessions = stmt
        .query_map([], |row| {
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                worktree_id: row.get(2)?,
                title: row.get(3)?,
                default_config: row.get(4)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(sessions)
}

/// Find sessions by project
pub fn find_by_project(conn: &Connection, project_id: i32) -> Result<Vec<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, worktree_id, title, default_config, created_at, updated_at
         FROM sessions WHERE project_id = ?1 ORDER BY updated_at DESC",
    )?;

    let sessions = stmt
        .query_map([project_id], |row| {
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                worktree_id: row.get(2)?,
                title: row.get(3)?,
                default_config: row.get(4)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(sessions)
}

/// Find sessions by worktree
#[allow(dead_code)]
pub fn find_by_worktree(conn: &Connection, worktree_id: i32) -> Result<Vec<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, worktree_id, title, default_config, created_at, updated_at
         FROM sessions WHERE worktree_id = ?1 ORDER BY updated_at DESC",
    )?;

    let sessions = stmt
        .query_map([worktree_id], |row| {
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                worktree_id: row.get(2)?,
                title: row.get(3)?,
                default_config: row.get(4)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(sessions)
}

/// Find session by ID
pub fn find_by_id(conn: &Connection, session_id: &str) -> Result<Option<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, worktree_id, title, default_config, created_at, updated_at FROM sessions WHERE id = ?1"
    )?;

    let session = stmt
        .query_row([session_id], |row| {
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                worktree_id: row.get(2)?,
                title: row.get(3)?,
                default_config: row.get(4)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            })
        })
        .optional()?;

    Ok(session)
}
