---
date: 2025-07-18
---

# Claude CLI Session Management Research

## Overview

This document describes the behavior of Claude CLI session management, particularly how `--resume`, `--continue`, and `--session-id` options work when integrating with external applications.

## Key Findings

### Session ID Behavior

1. **New Session Creation**: Every time Claude CLI is invoked with `--resume` or `--continue`, it creates a new session file with a new session ID, even though it preserves the conversation history.

2. **Session File Location**: Session files are stored in `~/.claude/projects/{project-path}/` as JSONL files with UUID filenames.

3. **History Preservation**: When using `--resume` with a session ID, Claude CLI:
   - Creates a new session file with a new UUID
   - Copies all conversation history from the resumed session
   - Continues the conversation with the full context

### Command Options

#### `--continue`

- Continues the most recent conversation
- Creates a new session file with a new session ID
- Preserves full conversation history

#### `--resume [sessionId]`

- Resumes a specific conversation by session ID
- Creates a new session file with a new session ID
- Copies all history from the specified session

#### `--session-id <uuid>`

- Attempts to use a specific session ID for a new conversation
- Fails with error "Session ID is already in use" if the ID exists
- Cannot be used to continue existing sessions

### Process Constraints

- **Single Process per Session ID**: Only one Claude CLI process can use a specific session ID at a time
- **New Process = New Session**: Each new CLI process invocation creates a new session, even with resume options

### Observed Behavior Example

```bash
# First message
claude --print "hi"
# Returns session ID: abc123

# Second message with --resume
claude --print --resume abc123 "hello"
# Creates NEW session ID: def456
# But includes "hi" -> response -> "hello" in history

# Third message with --continue
claude --print --continue "test"
# Creates NEW session ID: ghi789
# But includes full conversation history
```

## Implementation Implications

### Current Challenges

1. **Session ID Inconsistency**: Each message creates a new Claude session ID, making it difficult to track conversations by session ID alone
2. **Process Overhead**: Starting a new CLI process for each message is inefficient
3. **Database Sync**: Need to constantly update the database with new Claude session IDs

### Potential Solutions

1. **Accept the Behavior**:
   - Track Agentia session ID separately from Claude session IDs
   - Maintain a 1-to-many relationship between Agentia sessions and Claude sessions
   - Focus on conversation continuity rather than session ID consistency

2. **Long-Running Process** (Not tested):
   - Keep Claude CLI process running
   - Send messages via stdin
   - Maintain single session throughout conversation

3. **Interactive Mode** (Not tested):
   - Use Claude CLI in interactive mode without `--print`
   - Manage input/output streams programmatically

## Technical Details

### Session File Format

Session files are JSONL (JSON Lines) format containing:

- User messages
- Assistant responses
- Metadata (timestamps, UUIDs, session IDs)
- Tool usage information

### History Duplication

When resuming a session, all messages get the same new timestamp, indicating they were copied:

```json
{ "timestamp": "2025-07-17T10:57:11.787Z" } // All messages have same timestamp
```

## Recommendations

For the current implementation:

1. Accept that Claude CLI will create new session IDs for each message
2. Use `--continue` option for conversation continuity
3. Track the latest Claude session ID in the database
4. Focus on conversation history preservation rather than session ID consistency

## Future Considerations

- Investigate Claude CLI's interactive mode for persistent sessions
- Consider implementing a message queue with a single long-running CLI process
- Explore Claude CLI's API or SDK alternatives if available
