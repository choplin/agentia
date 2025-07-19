# Session Management Design

## Overview

This document describes the session management architecture for Agentia, focusing on the relationship between Agentia Sessions and Claude CLI Sessions.

## Core Concepts

1. **Agentia Session**: A logical grouping of work, representing a user's conceptual task or project phase
2. **Claude CLI Session**: An actual CLI process execution instance
3. **Relationship**: One Agentia Session contains multiple Claude CLI Sessions (1:n relationship)

## Data Model

### Sessions Table (Agentia Sessions)

```sql
Sessions {
    id: TEXT PRIMARY KEY,                -- UUID
    project_id: INTEGER NOT NULL,
    worktree_id: INTEGER NOT NULL,
    title: TEXT NOT NULL,
    default_config: TEXT,                -- JSON (ClaudeSessionConfig)
    created_at: DATETIME NOT NULL,
    updated_at: DATETIME NOT NULL,

    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (worktree_id) REFERENCES worktrees(id)
}
```

### ClaudeCliSessions Table (Execution History)

```sql
ClaudeCliSessions {
    id: INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id: TEXT NOT NULL,           -- Agentia Session ID
    claude_session_id: TEXT NOT NULL,    -- Claude's internal session ID
    file_path: TEXT NOT NULL,            -- ~/.claude/projects/.../xxx.jsonl
    config: TEXT NOT NULL,               -- JSON (actual config used)
    parent_cli_session_id: INTEGER,      -- Parent session (when resumed with -r)
    process_pid: INTEGER,                -- PID if running
    exit_code: INTEGER,                  -- Process exit code
    started_at: DATETIME NOT NULL,
    ended_at: DATETIME,

    FOREIGN KEY (session_id) REFERENCES sessions(id),
    FOREIGN KEY (parent_cli_session_id) REFERENCES claude_cli_sessions(id)
}
```

### ActiveCliSessions Table (Running Processes)

```sql
ActiveCliSessions {
    session_id: TEXT NOT NULL,
    cli_session_id: INTEGER NOT NULL,
    started_at: DATETIME NOT NULL,

    PRIMARY KEY (session_id, cli_session_id),
    FOREIGN KEY (session_id) REFERENCES sessions(id),
    FOREIGN KEY (cli_session_id) REFERENCES claude_cli_sessions(id)
}
```

## Design Principles

### 1. Message Storage

- Messages are NOT stored in the database
- Messages are read from Claude CLI's JSONL files
- Database only stores reference information

### 2. Configuration Hierarchy

Configuration can be inherited in the following order:

1. Project default_config
2. Session default_config
3. ClaudeCliSession config (actual runtime config)

Each level can override settings from the previous level.

### 3. Multiple Concurrent Executions

- ActiveCliSessions table tracks running processes
- Allows A/B testing scenarios within the same Agentia Session
- Example: Testing different approaches in parallel

### 4. History Tracking

- `parent_cli_session_id` records the resumption chain
- First execution: parent_cli_session_id = NULL
- Subsequent executions: Always resume previous with `-r` flag
- Creates a linear history (or branching for A/B tests)

## Lifecycle Management

### 1. New Agentia Session

```
User creates session → Insert into Sessions table
Status: No active CLI sessions
```

### 2. First Message

```
User sends message → Spawn new ClaudeCliProcess
→ Insert into ClaudeCliSessions (parent_cli_session_id = NULL)
→ Insert into ActiveCliSessions
```

### 3. Continued Conversation

```
Messages exchanged via stdin/stdout
→ Appended to JSONL file
→ No database changes
```

### 4. Process Termination

```
Process exits → Detect exit code
→ Update ClaudeCliSessions (exit_code, ended_at)
→ Delete from ActiveCliSessions
```

### 5. Resume Session

```
User sends new message → Spawn new ClaudeCliProcess with -r
→ Insert into ClaudeCliSessions (parent_cli_session_id = previous)
→ Insert into ActiveCliSessions
```

## State Management

### Session State

- Agentia Sessions have no explicit state field
- State is derived from active CLI sessions

### Process State

Determined by ClaudeCliSessions fields:

- Running: process_pid NOT NULL, ended_at NULL
- Exited: exit_code = 0, ended_at NOT NULL
- Failed: exit_code != 0, ended_at NOT NULL

### UI Display State

```rust
// Pseudo-code for deriving display status
if (ActiveCliSessions has entries for session) {
    return "Running"
} else if (last ClaudeCliSession has exit_code = 0) {
    return "Exited"
} else if (last ClaudeCliSession has exit_code != 0) {
    return "Failed"
} else {
    return "Idle"
}
```

## Configuration Management

### ClaudeSessionConfig Structure

```rust
struct ClaudeSessionConfig {
    model: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    permission_mode: Option<String>,
    working_directory: Option<String>,
}
```

All fields are optional to support inheritance and partial overrides.

### Config Merging

Configs are merged in priority order (later overrides earlier):

1. Application defaults
2. Project default_config
3. Session default_config
4. Runtime config

## Query Examples

### Get Active Sessions for an Agentia Session

```sql
SELECT c.*
FROM claude_cli_sessions c
JOIN active_cli_sessions a ON c.id = a.cli_session_id
WHERE a.session_id = ?
```

### Get Session History

```sql
SELECT * FROM claude_cli_sessions
WHERE session_id = ?
ORDER BY started_at DESC
```

### Get Sessions with Status

```sql
SELECT s.*,
       COUNT(a.cli_session_id) as active_count
FROM sessions s
LEFT JOIN active_cli_sessions a ON s.id = a.session_id
GROUP BY s.id
```

## Future Considerations

1. **Branching Visualization**: The parent_cli_session_id enables building a tree view of session evolution
2. **Performance Metrics**: Can add duration, token usage, etc. to ClaudeCliSessions
3. **Cleanup Policy**: Old JSONL files and database records retention policy
