---
created: 2025-08-01
updated: 2025-08-01
---

# State Management Architecture

## Core Principle: Backend as Single Source of Truth

In Tauri applications, the backend (Rust) should be the single source of truth for all application state. The frontend should not duplicate backend state in stores.

## Architecture Guidelines

### ❌ Don't: Duplicate Backend State in Frontend

```typescript
// Bad: Duplicating backend state
export const sessions = writable<Session[]>([]);
export const projects = writable<Project[]>([]);
```

Problems:

- State synchronization issues
- Memory waste
- Potential data inconsistency
- Complex update logic

### ✅ Do: Fetch Data When Needed

```typescript
// Good: Fetch from backend when needed
async function getRunningSessionsForSidebar() {
  return await invoke("list_sessions", { status: "running" });
}
```

### ✅ Do: Use Events for Real-time Updates

```typescript
// Good: Listen for backend events and re-fetch
listen("session-state-changed", async () => {
  // Don't store, just re-fetch
  const runningSessions = await invoke("list_running_sessions");
  updateUI(runningSessions);
});
```

## What Should Be in Stores?

Only **UI-specific state** that doesn't exist in the backend:

### ✅ Good Store Examples

1. **Theme Store** (`theme.ts`)
   - Light/Dark/System theme preference
   - Purely UI concern
   - Persisted in localStorage

2. **UI Layout State**
   - Sidebar collapsed/expanded
   - Panel sizes
   - View modes

3. **Temporary UI State**
   - Form inputs before submission
   - UI loading states
   - Error messages

### ❌ Bad Store Examples

1. **Session Store** - Backend manages sessions
2. **Project Store** - Backend manages projects
3. **User Store** - Backend manages user data

## Implementation Pattern

### Component-Level Data Fetching

```typescript
// In +page.svelte or component
<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  let sessions = $state<Session[]>([]);

  onMount(async () => {
    // Initial fetch
    sessions = await invoke('list_sessions');

    // Listen for updates
    const unlisten = await listen('sessions-updated', async () => {
      sessions = await invoke('list_sessions');
    });

    return () => {
      unlisten();
    };
  });
</script>
```

### Shared Data Between Components

If multiple components need the same backend data:

1. **Parent-Child**: Pass via props
2. **Siblings**: Lift state to common parent
3. **Distant Components**: Use backend events

```typescript
// Component A: Create session
await invoke("create_session", { title: "New Session" });
// Backend emits 'session-created' event

// Component B: Sidebar listening
listen("session-created", async () => {
  runningSessions = await invoke("list_running_sessions");
});
```

## Benefits

1. **Simpler Architecture**: No state synchronization logic
2. **Better Performance**: Less memory usage
3. **Consistency**: Backend is always authoritative
4. **Easier Testing**: Components just render data
5. **Clear Boundaries**: UI state vs Business state

## Migration Guide

When refactoring from stores to direct backend calls:

1. Remove store files
2. Replace store subscriptions with local state
3. Add `onMount` fetching
4. Set up event listeners for updates
5. Update components to use local state

Example migration:

```typescript
// Before: Using store
import { sessions } from '$lib/stores/session';
{#each $sessions as session}

// After: Local state
let sessions = $state<Session[]>([]);
onMount(async () => {
  sessions = await invoke('list_sessions');
});
{#each sessions as session}
```
