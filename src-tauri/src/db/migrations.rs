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
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                settings TEXT
            );

            CREATE TABLE IF NOT EXISTS worktrees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                is_main INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY NOT NULL,
                project_id INTEGER NOT NULL,
                worktree_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                default_config TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
                FOREIGN KEY (worktree_id) REFERENCES worktrees(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS claude_cli_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                claude_session_id TEXT NOT NULL,
                file_path TEXT NOT NULL,
                config TEXT NOT NULL,
                parent_cli_session_id INTEGER,
                process_pid INTEGER,
                exit_code INTEGER,
                started_at TEXT NOT NULL,
                ended_at TEXT,
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
                FOREIGN KEY (parent_cli_session_id) REFERENCES claude_cli_sessions(id)
            );

            CREATE TABLE IF NOT EXISTS active_cli_sessions (
                session_id TEXT NOT NULL,
                cli_session_id INTEGER NOT NULL,
                started_at TEXT NOT NULL,
                PRIMARY KEY (session_id, cli_session_id),
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
                FOREIGN KEY (cli_session_id) REFERENCES claude_cli_sessions(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_worktrees_project_id ON worktrees(project_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_project_id ON sessions(project_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_worktree_id ON sessions(worktree_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_created_at ON sessions(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_claude_cli_sessions_session_id ON claude_cli_sessions(session_id);
            CREATE INDEX IF NOT EXISTS idx_claude_cli_sessions_started_at ON claude_cli_sessions(started_at DESC);
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
