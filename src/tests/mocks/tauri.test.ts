import { describe, it, expect } from "vitest";

describe("Tauri Mock", () => {
  it("should export TauriMock class", async () => {
    const module = await import("./tauri");
    expect(module.TauriMock).toBeDefined();
    expect(typeof module.TauriMock).toBe("function");
  });

  it("should create TauriMock instance", async () => {
    const { TauriMock } = await import("./tauri");
    const mock = new TauriMock();
    expect(mock).toBeDefined();
    expect(mock.invoke).toBeDefined();
    expect(mock.listen).toBeDefined();
    expect(mock.emit).toBeDefined();
    expect(mock.reset).toBeDefined();
  });
});
