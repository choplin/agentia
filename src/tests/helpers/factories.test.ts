import { describe, it, expect } from "vitest";

describe("Test Data Factories", () => {
  it("should export createSession function", async () => {
    const module = await import("./factories");
    expect(module.createSession).toBeDefined();
    expect(typeof module.createSession).toBe("function");
  });

  it("should export createProject function", async () => {
    const module = await import("./factories");
    expect(module.createProject).toBeDefined();
    expect(typeof module.createProject).toBe("function");
  });

  it("should export createWorktree function", async () => {
    const module = await import("./factories");
    expect(module.createWorktree).toBeDefined();
    expect(typeof module.createWorktree).toBe("function");
  });
});
