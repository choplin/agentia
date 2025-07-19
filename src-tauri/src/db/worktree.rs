use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};

use super::models::Worktree;

pub fn create(
    conn: &Connection,
    project_id: i32,
    path: &str,
    name: &str,
    is_main: bool,
) -> Result<Worktree> {
    let now = Utc::now();

    conn.execute(
        "INSERT INTO worktrees (project_id, path, name, is_main, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![project_id, path, name, i32::from(is_main), now.to_rfc3339(), now.to_rfc3339()],
    )?;

    let id = i32::try_from(conn.last_insert_rowid())
        .map_err(|_| anyhow::anyhow!("Row ID too large for i32"))?;

    Ok(Worktree {
        id,
        project_id,
        path: path.to_string(),
        name: name.to_string(),
        is_main,
        created_at: now,
        updated_at: now,
    })
}

pub fn find_by_project(conn: &Connection, project_id: i32) -> Result<Vec<Worktree>> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, path, name, is_main, created_at, updated_at
         FROM worktrees
         WHERE project_id = ?1
         ORDER BY is_main DESC, name ASC",
    )?;

    let worktrees = stmt
        .query_map(params![project_id], |row| {
            Ok(Worktree {
                id: row.get(0)?,
                project_id: row.get(1)?,
                path: row.get(2)?,
                name: row.get(3)?,
                is_main: row.get::<_, i32>(4)? != 0,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(worktrees)
}

#[allow(dead_code)]
pub fn find_by_id(conn: &Connection, id: i32) -> Result<Option<Worktree>> {
    let result = conn
        .query_row(
            "SELECT id, project_id, path, name, is_main, created_at, updated_at
         FROM worktrees
         WHERE id = ?1",
            params![id],
            |row| {
                Ok(Worktree {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    path: row.get(2)?,
                    name: row.get(3)?,
                    is_main: row.get::<_, i32>(4)? != 0,
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                })
            },
        )
        .optional()?;

    Ok(result)
}

#[allow(dead_code)]
pub fn find_by_path(conn: &Connection, path: &str) -> Result<Option<Worktree>> {
    let result = conn
        .query_row(
            "SELECT id, project_id, path, name, is_main, created_at, updated_at
         FROM worktrees
         WHERE path = ?1",
            params![path],
            |row| {
                Ok(Worktree {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    path: row.get(2)?,
                    name: row.get(3)?,
                    is_main: row.get::<_, i32>(4)? != 0,
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                })
            },
        )
        .optional()?;

    Ok(result)
}

#[allow(dead_code)]
pub fn delete(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM worktrees WHERE id = ?1", params![id])?;
    Ok(())
}
