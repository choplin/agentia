/**
 * Test data factory functions for creating consistent test data
 */

import type { Session, Project, Worktree } from "$lib/types";

/**
 * Create a test session with default values
 */
export function createSession(overrides?: Partial<Session>): Session {
  return {
    id: "test-session-1",
    title: "Test Session",
    config: {
      model: "claude-3-5-sonnet-20241022",
      temperature: 0.5,
      maxTokens: 4096,
      permissionMode: "ask",
      workingDirectory: "/test/project/path",
    },
    messages: [],
    status: { type: "running" },
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
    claudeSessionId: "test-claude-session-1",
    ...overrides,
  };
}

/**
 * Create a test project with default values
 */
export function createProject(overrides?: Partial<Project>): Project {
  return {
    id: 1,
    name: "Test Project",
    path: "/test/project/path",
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
    ...overrides,
  };
}

/**
 * Create a test worktree with default values
 */
export function createWorktree(overrides?: Partial<Worktree>): Worktree {
  return {
    id: 1,
    projectId: 1,
    name: "main",
    path: "/test/project/path",
    isMain: true,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
    ...overrides,
  };
}
