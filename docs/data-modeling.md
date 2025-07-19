# Agentia Data Modeling Design

## 1. Conceptual Model

```
Project (Git repository)
    ├── Worktree (Working directory)
    │   ├── Session (Claude CLI session)
    │   ├── Session
    │   └── Session
    └── Worktree
        └── Session
```

## 2. Logical Model

```sql
-- Projects: Represents a Git repository
Projects {
    id: INTEGER PRIMARY KEY AUTOINCREMENT,
    path: TEXT UNIQUE NOT NULL,      -- Git root path (where .git exists)
    name: TEXT NOT NULL,             -- Display name
    created_at: DATETIME NOT NULL,
    updated_at: DATETIME NOT NULL
}

-- Worktrees: Git worktree or main working directory
Worktrees {
    id: INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id: INTEGER NOT NULL,
    path: TEXT UNIQUE NOT NULL,      -- Actual working directory path
    name: TEXT NOT NULL,             -- Display name (main, feature-x, etc.)
    is_main: BOOLEAN NOT NULL,       -- Whether this is the main worktree
    created_at: DATETIME NOT NULL,
    updated_at: DATETIME NOT NULL,

    FOREIGN KEY (project_id) REFERENCES projects(id)
}

-- Sessions: Claude CLI conversation sessions
Sessions {
    id: TEXT PRIMARY KEY,            -- UUID
    project_id: INTEGER NOT NULL,    -- Redundant column for query optimization
    worktree_id: INTEGER NOT NULL,
    title: TEXT NOT NULL,
    config: TEXT,                    -- JSON (model, temperature, etc.)
    claude_session_id: TEXT,         -- Claude CLI session file path
    created_at: DATETIME NOT NULL,
    updated_at: DATETIME NOT NULL,

    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (worktree_id) REFERENCES worktrees(id)
}
```

## 3. Physical Model (Indexes)

```sql
CREATE INDEX idx_worktrees_project_id ON worktrees(project_id);
CREATE INDEX idx_sessions_project_id ON sessions(project_id);
CREATE INDEX idx_sessions_worktree_id ON sessions(worktree_id);
CREATE INDEX idx_sessions_created_at ON sessions(created_at DESC);
```

## 4. Constraints and Rules

### 4.1 Project.path

- Must be a path where `.git` directory exists
- Normalized (absolute path)

### 4.2 Worktree.path

- Must be an existing directory
- Unique within the project

### 4.3 Worktree.is_main

- Each project must have exactly one main worktree
- Main worktree cannot be deleted if other worktrees exist

### 4.4 Session.claude_session_id

- Format: `~/.claude/projects/{encoded-path}/{uuid}.jsonl`
- NULL for new sessions

## 5. External System Mapping

### 5.1 Git Integration

- Project.path ← `git rev-parse --show-toplevel`
- Worktree ← `git worktree list`

### 5.2 Claude CLI Integration

- Session ← `~/.claude/projects/*/**.jsonl`
- Message history is read from JSONL files

## 6. Data Consistency Policy

### 6.1 Basic Principles

- Database is the Source of Truth
- Filesystem state is checked on reference
- User confirmation required for inconsistencies

### 6.2 Consistency Check Timing

- App startup: Lightweight check
- Project selection: Validate project worktrees
- Session creation: Verify selected worktree exists
- Session list display: Check Claude CLI file existence

### 6.3 Handling Inconsistencies

- Project path doesn't exist → Invalidate project or prompt for new path
- Worktree path doesn't exist → Invalidate or remove worktree
- Claude CLI file missing → Treat session as historical, create new CLI session on message send
