use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

use crate::db::models::Session;

pub struct SessionRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SessionRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(
        &self,
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

        self.conn.execute(
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

    pub fn update_claude_session_id(
        &self,
        session_id: &str,
        claude_session_id: &str,
    ) -> Result<()> {
        self.conn.execute(
            "UPDATE sessions SET claude_session_id = ?1, updated_at = ?2 WHERE id = ?3",
            params![claude_session_id, chrono::Utc::now(), session_id],
        )?;
        Ok(())
    }

    pub fn find_by_project(&self, project_id: &str) -> Result<Vec<Session>> {
        let mut stmt = self.conn.prepare(
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

    #[allow(dead_code)]
    pub fn find_by_id(&self, session_id: &str) -> Result<Option<Session>> {
        let mut stmt = self.conn.prepare(
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
}
