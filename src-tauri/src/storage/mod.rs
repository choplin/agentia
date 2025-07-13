use crate::claude_code::Session;
use anyhow::Result;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

#[allow(dead_code)]
pub struct Storage {
    data_dir: PathBuf,
}

#[allow(dead_code)]
impl Storage {
    pub fn new(app: &AppHandle) -> Result<Self> {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| anyhow::anyhow!("Failed to get app data dir: {}", e))?;

        // Create sessions directory
        let sessions_dir = data_dir.join("sessions");
        std::fs::create_dir_all(&sessions_dir)?;

        Ok(Self { data_dir })
    }

    pub async fn save_session(&self, session: &Session) -> Result<()> {
        let path = self.data_dir.join("sessions").join(format!("{}.json", session.id));
        let json = serde_json::to_string_pretty(session)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    pub async fn load_session(&self, id: Uuid) -> Result<Option<Session>> {
        let path = self.data_dir.join("sessions").join(format!("{id}.json"));

        if !path.exists() {
            return Ok(None);
        }

        let json = tokio::fs::read_to_string(path).await?;
        let session: Session = serde_json::from_str(&json)?;
        Ok(Some(session))
    }

    pub async fn list_sessions(&self) -> Result<Vec<Session>> {
        let sessions_dir = self.data_dir.join("sessions");
        let mut sessions = Vec::new();

        let mut entries = tokio::fs::read_dir(sessions_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(json) = tokio::fs::read_to_string(entry.path()).await {
                    if let Ok(session) = serde_json::from_str::<Session>(&json) {
                        sessions.push(session);
                    }
                }
            }
        }

        sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(sessions)
    }

    pub async fn delete_session(&self, id: Uuid) -> Result<()> {
        let path = self.data_dir.join("sessions").join(format!("{id}.json"));
        if path.exists() {
            tokio::fs::remove_file(path).await?;
        }
        Ok(())
    }
}
