# CLAUDE.md - Project-Specific Instructions for Agentia

## UI Development Guidelines

### Language

- All UI text should be in English
- Code comments can be in English or Japanese

### List Display Format

- Use table format for all list views (Sessions, Projects, Worktrees, MCP servers, etc.)
- Avoid card-based layouts for lists
- Keep consistent table structure across all pages

### Routing Architecture

- Use proper SvelteKit routing instead of SPA-style navigation
- Each major section should have its own route
- Shared layout components go in `+layout.svelte`

## Technical Guidelines

### State Management Architecture

- **Backend is Single Source of Truth** - Don't duplicate backend state in frontend stores
- **Stores are for UI state only** - Theme, layout preferences, temporary form state
- **Fetch data when needed** - Use `invoke()` to get data from backend
- **Use events for updates** - Listen to backend events and re-fetch data
- See `docs/state-management-architecture.md` for detailed guidelines

### Svelte 5 Runes Mode

- Use `$state()` for reactive state
- Use `$derived()` for computed values
- Use `$props()` for component props
- Use `{@render children?.()}` instead of `<slot>` in layouts

### Component Library

- Use shadcn-svelte components where available
- Follow the existing pattern for UI components
- Icons: Use lucide-svelte (not emoji icons)

### Git Commit Style

- Follow conventional commits format
- Keep commit messages focused on what changed, not implementation details
- Don't include bug fixes or compatibility updates in commit messages

### Backend Integration

- Tauri commands use snake_case (e.g., `get_projects`, `list_sessions`)
- Use `@tauri-apps/api/core` invoke function for API calls
- Real-time events follow format: `session-{session_id}-message`

## Project Structure

### Frontend

- Framework: SvelteKit with Svelte 5 (runes mode)
- UI: Tailwind CSS + shadcn-svelte
- Language: TypeScript

### Backend

- Framework: Tauri v2 with Rust
- Database: SQLite with rusqlite
- Integration: Claude Code CLI

## Development Workflow

### Before Committing

**ALWAYS run the precommit check before creating commits:**

```bash
pnpm run precommit
```

This command will:

- Format code with prettier and rustfmt
- Run eslint checks
- Run svelte-check
- Run cargo clippy

Individual commands are also available:

- `pnpm run fmt` - Format code (TypeScript & Rust)
- `pnpm run lint` - Run all lint checks
- `pnpm run check` - Run svelte-check only

### Pre-commit Hooks

- prettier, eslint, svelte-check are configured
- All files are auto-formatted on commit
- Accessibility checks are enforced

### Testing

- Check for test commands in package.json before running tests
- Don't assume specific test frameworks

## Design Principles

### User Mental Model

Project → Worktree → Session

### UI Layout

- Activity bar for main navigation
- Side panel for quick access to running/recent sessions
- Filters in main area toolbar (not in sidebar)
- Keep UI simple initially, add features based on usage feedback

### Status Display

- Use consistent badge styles for status indicators
- Active/Inactive, Running/Stopped, Connected/Disconnected, etc.
- Place action buttons in the rightmost column of tables
