use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

use super::models::Project;

/// Create a new project
pub fn create(conn: &Connection, path: &str, name: &str) -> Result<Project> {
    let project = Project {
        id: Uuid::new_v4().to_string(),
        path: path.to_string(),
        name: name.to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        settings: None,
    };

    conn.execute(
        "INSERT INTO projects (id, path, name, created_at, updated_at, settings)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            project.id,
            project.path,
            project.name,
            project.created_at,
            project.updated_at,
            project.settings,
        ],
    )?;

    Ok(project)
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
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                settings: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(projects)
}

/// Find project by path
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
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                settings: row.get(5)?,
            })
        })
        .optional()?;

    Ok(project)
}

/// Find project by ID
pub fn find_by_id(conn: &Connection, id: &str) -> Result<Option<Project>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, name, created_at, updated_at, settings FROM projects WHERE id = ?1",
    )?;

    let project = stmt
        .query_row([id], |row| {
            Ok(Project {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                settings: row.get(5)?,
            })
        })
        .optional()?;

    Ok(project)
}
