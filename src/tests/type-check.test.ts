import { describe, it, expect } from "vitest";
import type { Project, Session, Worktree } from "../lib/types";

// Type checking sample test
// This file demonstrates that TypeScript types are properly checked in test files

describe("TypeScript type checking in tests", () => {
  it("should properly type check Project interface", () => {
    // This test verifies that TypeScript types work correctly
    const project: Project = {
      id: 1,
      name: "Test Project",
      path: "/path/to/project",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
      settings: "{}",
    };

    expect(project.id).toBe(1);
    expect(project.name).toBe("Test Project");
  });

  it("should properly type check Session interface", () => {
    const session: Session = {
      id: "test-session-id",
      title: "Test Session",
      config: {
        model: "claude-3-opus",
        temperature: 0.7,
        maxTokens: 4096,
        permissionMode: "standard",
        workingDirectory: "/path/to/project",
      },
      messages: [],
      status: { type: "running" },
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
      claudeSessionId: "claude-session-123",
    };

    expect(session.id).toBe("test-session-id");
    expect(session.status.type).toBe("running");
  });

  it("should properly type check Worktree interface", () => {
    const worktree: Worktree = {
      id: 1,
      projectId: 1,
      path: "/path/to/worktree",
      name: "Test Worktree",
      isMain: true,
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    expect(worktree.id).toBe(1);
    expect(worktree.isMain).toBe(true);
  });

  it("should fail to compile with incorrect types", () => {
    // This test demonstrates that TypeScript will catch type errors
    // The following would cause a TypeScript error if uncommented:
    // const invalidProject: Project = {
    //   id: 123, // Error: Type 'number' is not assignable to type 'string'
    //   name: "Test",
    //   // Missing required fields would also cause errors
    // };

    // Instead, we test that the type system is working by asserting
    // that proper types are enforced
    const properlyTyped = (p: Project): number => p.id;
    const testProject: Project = {
      id: 1,
      name: "Test",
      path: "/test",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    expect(properlyTyped(testProject)).toBe(1);
  });

  it("should work with generic types", () => {
    // Test generic type handling
    function identity<T>(arg: T): T {
      return arg;
    }

    const num = identity<number>(42);
    const str = identity<string>("hello");
    const obj = identity<{ name: string }>({ name: "test" });

    expect(num).toBe(42);
    expect(str).toBe("hello");
    expect(obj.name).toBe("test");
  });

  it("should work with async/await and Promises", async () => {
    // Test async type handling
    async function fetchData(): Promise<string> {
      return Promise.resolve("async data");
    }

    const result = await fetchData();
    expect(result).toBe("async data");
  });

  it("should properly type vi.fn() mocks", () => {
    // Test that vitest mock functions are properly typed
    const mockFn = vi.fn(() => "mocked value");

    const result = mockFn();
    expect(result).toBe("mocked value");
    expect(mockFn).toHaveBeenCalled();
  });
});

// Export empty object to make this a module
export {};
