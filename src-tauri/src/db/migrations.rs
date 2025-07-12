use anyhow::Result;
use rusqlite::Connection;

pub struct Migration {
    pub version: i32,
    pub description: &'static str,
    pub sql: &'static str,
}

pub const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    description: "Initial schema",
    sql: r"
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY NOT NULL,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                settings TEXT
            );

            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY NOT NULL,
                project_id TEXT NOT NULL,
                claude_session_id TEXT,
                title TEXT NOT NULL,
                config TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_project_id ON sessions(project_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_claude_session_id ON sessions(claude_session_id);
        ",
}];

pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Create migrations table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            version INTEGER PRIMARY KEY NOT NULL,
            description TEXT NOT NULL,
            applied_at TEXT NOT NULL
        )",
        [],
    )?;

    // Get current version
    let current_version = get_current_version(conn);

    // Run pending migrations
    for migration in MIGRATIONS {
        if migration.version > current_version {
            println!("Running migration v{}: {}", migration.version, migration.description);

            conn.execute_batch(migration.sql)?;

            conn.execute(
                "INSERT INTO migrations (version, description, applied_at) VALUES (?1, ?2, ?3)",
                [
                    &migration.version.to_string(),
                    migration.description,
                    &chrono::Utc::now().to_rfc3339(),
                ],
            )?;
        }
    }

    Ok(())
}

pub fn get_current_version(conn: &Connection) -> i32 {
    conn.query_row("SELECT COALESCE(MAX(version), 0) FROM migrations", [], |row| row.get(0))
        .unwrap_or(0)
}
