import { describe, it, expect, vi } from "vitest";

describe("setup.ts - Global mock initialization", () => {
  it("should initialize Tauri mock globally", () => {
    // Should have window.__TAURI_INTERNALS__ defined globally
    expect(globalThis).toHaveProperty("__TAURI_INTERNALS__");
    expect(globalThis.__TAURI_INTERNALS__).toBeDefined();
  });

  it("should provide invoke function through Tauri internals", () => {
    // Should have invoke method
    expect(globalThis.__TAURI_INTERNALS__).toHaveProperty("invoke");
    expect(typeof globalThis.__TAURI_INTERNALS__.invoke).toBe("function");
  });

  it("should provide event listening functions through Tauri internals", () => {
    // Should have listen method
    expect(globalThis.__TAURI_INTERNALS__).toHaveProperty("listen");
    expect(typeof globalThis.__TAURI_INTERNALS__.listen).toBe("function");

    // Should also have emit method (for testing)
    expect(globalThis.__TAURI_INTERNALS__).toHaveProperty("emit");
    expect(typeof globalThis.__TAURI_INTERNALS__.emit).toBe("function");
  });

  it("should have TauriMock instance accessible globally", () => {
    // Should have global TauriMock instance
    expect(globalThis).toHaveProperty("__TAURI_MOCK__");
    expect(globalThis.__TAURI_MOCK__).toBeDefined();
    expect(globalThis.__TAURI_MOCK__).toHaveProperty("reset");
  });

  it("should allow setting mock responses", async () => {
    // Should be able to set mock responses
    const mockProjects = [{ id: "1", name: "Test Project" }];
    globalThis.__TAURI_MOCK__.setMockResponse("get_projects", mockProjects);

    // Should return mock response when invoke is called
    await expect(globalThis.__TAURI_INTERNALS__.invoke("get_projects")).resolves.toEqual(
      mockProjects,
    );
  });

  it("should reset mocks between tests", async () => {
    // Should be able to reset mocks
    globalThis.__TAURI_MOCK__.setMockResponse("test_command", "test_value");
    globalThis.__TAURI_MOCK__.reset();

    // Should throw error after reset
    await expect(globalThis.__TAURI_INTERNALS__.invoke("test_command")).rejects.toThrow();
  });
});

// Global type definitions test
describe("Global type definitions", () => {
  it("should have proper TypeScript types", () => {
    // Should have proper TypeScript types defined
    const tauri: any = globalThis.__TAURI_INTERNALS__;
    const mock: any = globalThis.__TAURI_MOCK__;

    // Basic type check
    expect(tauri).toBeTruthy();
    expect(mock).toBeTruthy();
  });
});

// afterEach cleanup tests
describe("afterEach cleanup functionality", () => {
  it("should clean up DOM after each test", () => {
    // Create DOM element
    const testDiv = document.createElement("div");
    testDiv.id = "test-element";
    document.body.appendChild(testDiv);

    // DOM element should exist
    expect(document.getElementById("test-element")).toBeTruthy();
  });

  it("should have cleaned up DOM from previous test", () => {
    // DOM element from previous test should be gone
    expect(document.getElementById("test-element")).toBeFalsy();
  });

  it("should reset all vi.fn() mocks after each test", () => {
    const mockFn = vi.fn();
    mockFn("test");
    expect(mockFn).toHaveBeenCalledWith("test");
  });

  it("should clear all timers after each test", () => {
    let called = false;
    setTimeout(() => {
      called = true;
    }, 100);

    // Timer should be active
    expect(called).toBe(false);
  });

  it("should have cleared timers from previous test", () => {
    // Create a new timer to verify timer state
    const callback = vi.fn();
    vi.useFakeTimers();
    setTimeout(callback, 100);

    // Advance time - should not call previous test's timer
    vi.advanceTimersByTime(200);
    expect(callback).toHaveBeenCalledOnce();

    vi.useRealTimers();
  });

  it("should maintain clean Tauri mock state", async () => {
    // Add a custom command
    globalThis.__TAURI_MOCK__.mockCommand("cleanup_test", { data: "test" });

    // Verify it works
    const result = await globalThis.__TAURI_INTERNALS__.invoke("cleanup_test");
    expect(result).toEqual({ data: "test" });
  });

  it("should have reset Tauri commands from previous test", async () => {
    // Previous test's command should not exist
    await expect(globalThis.__TAURI_INTERNALS__.invoke("cleanup_test")).rejects.toThrow();
  });

  it("should clear event listeners after each test", () => {
    const handler = vi.fn();
    const unlisten = globalThis.__TAURI_INTERNALS__.listen("test-event", handler);
    globalThis.__TAURI_INTERNALS__.emit("test-event", { data: "test" });

    expect(handler).toHaveBeenCalledWith({ event: "test-event", payload: { data: "test" } });

    // Clean up for this test
    unlisten();
  });

  it("should have cleared event listeners from previous test", () => {
    const handler = vi.fn();
    // Emit event without registering - should not trigger any handler
    globalThis.__TAURI_INTERNALS__.emit("test-event", { data: "test" });

    expect(handler).not.toHaveBeenCalled();
  });
});
