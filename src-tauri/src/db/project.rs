use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};

use super::models::Project;

/// Create a new project
pub fn create(conn: &Connection, path: &str, name: &str) -> Result<Project> {
    let now = Utc::now();

    conn.execute(
        "INSERT INTO projects (path, name, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4)",
        params![path, name, now.to_rfc3339(), now.to_rfc3339()],
    )?;

    let id = i32::try_from(conn.last_insert_rowid())
        .map_err(|_| anyhow::anyhow!("Row ID too large for i32"))?;

    Ok(Project {
        id,
        path: path.to_string(),
        name: name.to_string(),
        created_at: now,
        updated_at: now,
        settings: None,
    })
}

/// Get all projects
pub fn find_all(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, name, created_at, updated_at, settings FROM projects ORDER BY updated_at DESC"
    )?;

    let projects = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                settings: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(projects)
}

/// Find project by path
#[allow(dead_code)]
pub fn find_by_path(conn: &Connection, path: &str) -> Result<Option<Project>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, name, created_at, updated_at, settings FROM projects WHERE path = ?1",
    )?;

    let project = stmt
        .query_row([path], |row| {
            Ok(Project {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                settings: row.get(5)?,
            })
        })
        .optional()?;

    Ok(project)
}

/// Find project by ID
pub fn find_by_id(conn: &Connection, id: i32) -> Result<Option<Project>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, name, created_at, updated_at, settings FROM projects WHERE id = ?1",
    )?;

    let project = stmt
        .query_row([id], |row| {
            Ok(Project {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                settings: row.get(5)?,
            })
        })
        .optional()?;

    Ok(project)
}
