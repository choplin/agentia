use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};

use super::models::{ActiveCliSession, ClaudeCliSession};

pub fn create(
    conn: &Connection,
    session_id: &str,
    claude_session_id: &str,
    file_path: &str,
    config: &str,
    parent_cli_session_id: Option<i32>,
    process_pid: Option<i32>,
) -> Result<ClaudeCliSession> {
    let now = Utc::now();

    conn.execute(
        "INSERT INTO claude_cli_sessions
         (session_id, claude_session_id, file_path, config, parent_cli_session_id, process_pid, started_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            session_id,
            claude_session_id,
            file_path,
            config,
            parent_cli_session_id,
            process_pid,
            now.to_rfc3339()
        ],
    )?;

    let id = i32::try_from(conn.last_insert_rowid())
        .map_err(|_| anyhow::anyhow!("Row ID too large for i32"))?;

    Ok(ClaudeCliSession {
        id,
        session_id: session_id.to_string(),
        claude_session_id: claude_session_id.to_string(),
        file_path: file_path.to_string(),
        config: config.to_string(),
        parent_cli_session_id,
        process_pid,
        exit_code: None,
        started_at: now,
        ended_at: None,
    })
}

pub fn update_exit_status(conn: &Connection, id: i32, exit_code: i32) -> Result<()> {
    let now = Utc::now();

    conn.execute(
        "UPDATE claude_cli_sessions
         SET exit_code = ?1, ended_at = ?2, process_pid = NULL
         WHERE id = ?3",
        params![exit_code, now.to_rfc3339(), id],
    )?;

    Ok(())
}

#[allow(dead_code)]
pub fn find_by_session(conn: &Connection, session_id: &str) -> Result<Vec<ClaudeCliSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, session_id, claude_session_id, file_path, config,
            parent_cli_session_id, process_pid, exit_code, started_at, ended_at
         FROM claude_cli_sessions
         WHERE session_id = ?1
         ORDER BY started_at DESC",
    )?;

    let sessions = stmt
        .query_map(params![session_id], |row| {
            Ok(ClaudeCliSession {
                id: row.get(0)?,
                session_id: row.get(1)?,
                claude_session_id: row.get(2)?,
                file_path: row.get(3)?,
                config: row.get(4)?,
                parent_cli_session_id: row.get(5)?,
                process_pid: row.get(6)?,
                exit_code: row.get(7)?,
                started_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                ended_at: row.get::<_, Option<String>>(9)?.map(|s| {
                    chrono::DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&chrono::Utc)
                }),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(sessions)
}

pub fn find_latest_by_session(
    conn: &Connection,
    session_id: &str,
) -> Result<Option<ClaudeCliSession>> {
    let result = conn
        .query_row(
            "SELECT id, session_id, claude_session_id, file_path, config,
                parent_cli_session_id, process_pid, exit_code, started_at, ended_at
            FROM claude_cli_sessions
            WHERE session_id = ?1
            ORDER BY started_at DESC
            LIMIT 1",
            params![session_id],
            |row| {
                Ok(ClaudeCliSession {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    claude_session_id: row.get(2)?,
                    file_path: row.get(3)?,
                    config: row.get(4)?,
                    parent_cli_session_id: row.get(5)?,
                    process_pid: row.get(6)?,
                    exit_code: row.get(7)?,
                    started_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    ended_at: row.get::<_, Option<String>>(9)?.map(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .unwrap()
                            .with_timezone(&chrono::Utc)
                    }),
                })
            },
        )
        .optional()?;

    Ok(result)
}

#[allow(dead_code)]
pub fn find_by_id(conn: &Connection, id: i32) -> Result<Option<ClaudeCliSession>> {
    let result = conn
        .query_row(
            "SELECT id, session_id, claude_session_id, file_path, config,
                parent_cli_session_id, process_pid, exit_code, started_at, ended_at
            FROM claude_cli_sessions
            WHERE id = ?1",
            params![id],
            |row| {
                Ok(ClaudeCliSession {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    claude_session_id: row.get(2)?,
                    file_path: row.get(3)?,
                    config: row.get(4)?,
                    parent_cli_session_id: row.get(5)?,
                    process_pid: row.get(6)?,
                    exit_code: row.get(7)?,
                    started_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    ended_at: row.get::<_, Option<String>>(9)?.map(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .unwrap()
                            .with_timezone(&chrono::Utc)
                    }),
                })
            },
        )
        .optional()?;

    Ok(result)
}

// Active CLI Sessions management

pub fn create_active(conn: &Connection, session_id: &str, cli_session_id: i32) -> Result<()> {
    let now = Utc::now();

    conn.execute(
        "INSERT INTO active_cli_sessions (session_id, cli_session_id, started_at)
         VALUES (?1, ?2, ?3)",
        params![session_id, cli_session_id, now.to_rfc3339()],
    )?;

    Ok(())
}

pub fn delete_active(conn: &Connection, session_id: &str, cli_session_id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM active_cli_sessions
         WHERE session_id = ?1 AND cli_session_id = ?2",
        params![session_id, cli_session_id],
    )?;

    Ok(())
}

pub fn find_active_by_session(
    conn: &Connection,
    session_id: &str,
) -> Result<Vec<ActiveCliSession>> {
    let mut stmt = conn.prepare(
        "SELECT session_id, cli_session_id, started_at
         FROM active_cli_sessions
         WHERE session_id = ?1",
    )?;

    let sessions = stmt
        .query_map(params![session_id], |row| {
            Ok(ActiveCliSession {
                session_id: row.get(0)?,
                cli_session_id: row.get(1)?,
                started_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(sessions)
}

#[allow(dead_code)]
pub fn has_active_sessions(conn: &Connection, session_id: &str) -> Result<bool> {
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM active_cli_sessions WHERE session_id = ?1",
        params![session_id],
        |row| row.get(0),
    )?;

    Ok(count > 0)
}
