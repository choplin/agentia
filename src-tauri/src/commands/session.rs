use crate::claude_code::Session;
use crate::services::SessionService;
use crate::AppState;
use std::sync::Arc;
use uuid::Uuid;

#[tauri::command]
pub async fn create_session(
    state: tauri::State<'_, AppState>,
    project_id: i32,
    worktree_id: Option<i32>,
    title: Option<String>,
    default_config: Option<String>,
) -> Result<Session, String> {
    let service = SessionService::new(Arc::clone(&state.db));
    service.create_session(project_id, worktree_id, title, default_config).await
}

#[tauri::command]
pub async fn get_session(
    state: tauri::State<'_, AppState>,
    session_id: Uuid,
) -> Result<Option<Session>, String> {
    let service = SessionService::new(Arc::clone(&state.db));
    service.get_session(&session_id.to_string()).await
}

#[tauri::command]
pub async fn list_sessions(
    state: tauri::State<'_, AppState>,
    project_id: Option<i32>,
) -> Result<Vec<Session>, String> {
    let service = SessionService::new(Arc::clone(&state.db));
    service.list_sessions(project_id).await
}
