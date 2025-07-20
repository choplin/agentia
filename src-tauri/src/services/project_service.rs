use crate::db::{models::Project, Database};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ProjectService {
    db: Arc<Mutex<Database>>,
}

impl ProjectService {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, String> {
        self.db.lock().await.get_projects().map_err(|e| e.to_string())
    }

    pub async fn create_project(&self, path: &str, name: &str) -> Result<Project, String> {
        let db = self.db.lock().await;
        let project = db.create_project(path, name).map_err(|e| e.to_string())?;

        // Create main worktree for the new project
        db.create_worktree(project.id, path, "main", true).map_err(|e| e.to_string())?;

        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectService;
    use crate::db::Database;
    use std::sync::Arc;
    use tempfile::TempDir;
    use tokio::sync::Mutex;

    fn setup_test_db() -> (Arc<Mutex<Database>>, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new_with_path(&db_path).unwrap();
        (Arc::new(Mutex::new(db)), temp_dir)
    }

    #[tokio::test]
    async fn test_get_projects_empty() {
        let (db, _temp_dir) = setup_test_db();
        let service = ProjectService::new(Arc::clone(&db));

        let projects = service.get_projects().await.unwrap();
        assert!(projects.is_empty());
    }

    #[tokio::test]
    async fn test_create_and_get_projects() {
        let (db, _temp_dir) = setup_test_db();
        let service = ProjectService::new(Arc::clone(&db));

        // Create projects
        service.create_project("/test/project1", "Project 1").await.unwrap();
        service.create_project("/test/project2", "Project 2").await.unwrap();

        // Get all projects
        let all_projects = service.get_projects().await.unwrap();
        assert_eq!(all_projects.len(), 2);

        // Verify project details
        let first_project = all_projects.iter().find(|p| p.name == "Project 1").unwrap();
        assert_eq!(first_project.path, "/test/project1");

        let second_project = all_projects.iter().find(|p| p.name == "Project 2").unwrap();
        assert_eq!(second_project.path, "/test/project2");
    }

    #[tokio::test]
    async fn test_create_project_with_worktree() {
        let (db, _temp_dir) = setup_test_db();
        let service = ProjectService::new(Arc::clone(&db));

        // Create a project
        let project = service.create_project("/test/myproject", "My Project").await.unwrap();

        // Verify project was created
        assert_eq!(project.name, "My Project");
        assert_eq!(project.path, "/test/myproject");

        // Verify main worktree was created
        let db = db.lock().await;
        let worktrees = db.get_worktrees_by_project(project.id).unwrap();
        assert_eq!(worktrees.len(), 1);

        let main_worktree = &worktrees[0];
        assert_eq!(main_worktree.name, "main");
        assert_eq!(main_worktree.path, "/test/myproject");
        assert!(main_worktree.is_main);
    }

    #[tokio::test]
    async fn test_project_uniqueness() {
        let (db, _temp_dir) = setup_test_db();
        let service = ProjectService::new(Arc::clone(&db));

        // Create a project
        service.create_project("/test/unique", "Unique Project").await.unwrap();

        // Try to create another project with the same path
        let result = service.create_project("/test/unique", "Duplicate Project").await;

        // Should fail due to unique constraint
        assert!(result.is_err());
    }
}
