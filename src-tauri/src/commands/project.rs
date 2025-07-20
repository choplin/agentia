use crate::db::models::Project;
use crate::services::ProjectService;
use crate::AppState;
use std::sync::Arc;

#[tauri::command]
pub async fn get_projects(state: tauri::State<'_, AppState>) -> Result<Vec<Project>, String> {
    let service = ProjectService::new(Arc::clone(&state.db));
    service.get_projects().await
}

#[tauri::command]
pub async fn create_project(
    state: tauri::State<'_, AppState>,
    name: String,
    path: String,
) -> Result<Project, String> {
    let service = ProjectService::new(Arc::clone(&state.db));
    service.create_project(&path, &name).await
}
