/**
 * Vitest global setup file
 * This file is executed before each test file
 */

import { expect, vi, afterEach } from "vitest";
import { TauriMock } from "./mocks/tauri";

// Global configuration
declare global {
  var __TEST_MODE__: boolean;
  var __TAURI_INTERNALS__: {
    invoke: (cmd: string, args?: any) => Promise<any>;
    listen: (event: string, handler: (event: any) => void) => () => void;
    emit: (event: string, payload: any) => void;
  };
  var __TAURI_MOCK__: TauriMock;
}

// Set test mode flag
globalThis.__TEST_MODE__ = true;

// Initialize global mocks
const initializeGlobalMocks = () => {
  // Initialize Tauri mock
  const tauriMock = new TauriMock();
  globalThis.__TAURI_MOCK__ = tauriMock;

  // Mock implementation of Tauri internals
  // Provides the same interface as the actual Tauri API
  globalThis.__TAURI_INTERNALS__ = {
    invoke: (cmd: string, args?: any) => tauriMock.invoke(cmd, args),
    listen: (event: string, handler: (event: any) => void) => tauriMock.listen(event, handler),
    emit: (event: string, payload: any) => tauriMock.emit(event, payload),
  };

  // Provide mock reset functionality between tests
  // Expected to be called in afterEach hook
  afterEach(() => {
    tauriMock.reset();

    // Clear all DOM modifications
    document.body.innerHTML = "";
    document.head.innerHTML = "";

    // Clear all timers
    vi.clearAllTimers();

    // Reset all mocks
    vi.resetAllMocks();

    // Clear any custom properties added to window/global
    if (typeof window !== "undefined") {
      const win = window as any;
      Object.keys(win).forEach((key) => {
        if (key.startsWith("__test_") || key.startsWith("test_")) {
          delete win[key];
        }
      });
    }
  });
};

initializeGlobalMocks();

// Setup console method mocks
const setupConsoleMocks = () => {
  const originalError = console.error;
  const originalWarn = console.warn;

  // Mock console.error (suppress unnecessary error output during tests)
  console.error = vi.fn((...args) => {
    const message = args[0]?.toString() || "";

    // Patterns to ignore
    const ignoredPatterns = ["Svelte", "hydration", "Hydration"];
    if (ignoredPatterns.some((pattern) => message.includes(pattern))) {
      return;
    }

    originalError.apply(console, args);
  });

  // Mock console.warn (suppress unnecessary warnings during tests)
  console.warn = vi.fn((...args) => {
    const message = args[0]?.toString() || "";

    // Patterns to ignore
    const ignoredPatterns = ["deprecated", "experimental", "DEV"];
    if (ignoredPatterns.some((pattern) => message.includes(pattern))) {
      return;
    }

    originalWarn.apply(console, args);
  });
};

setupConsoleMocks();

// Global fetch mock (may be needed in the future)
if (!globalThis.fetch) {
  globalThis.fetch = vi.fn(() =>
    Promise.resolve({
      ok: true,
      status: 200,
      json: () => Promise.resolve({}),
      text: () => Promise.resolve(""),
    } as Response),
  );
}

// Browser API polyfills
const setupBrowserPolyfills = () => {
  // requestAnimationFrame polyfill (needed for jsdom)
  if (!globalThis.requestAnimationFrame) {
    globalThis.requestAnimationFrame = (cb: FrameRequestCallback): number => {
      return setTimeout(() => cb(Date.now()), 0) as unknown as number;
    };
  }

  if (!globalThis.cancelAnimationFrame) {
    globalThis.cancelAnimationFrame = (id: number): void => {
      clearTimeout(id);
    };
  }

  // ResizeObserver mock (may be used by Svelte components)
  if (!globalThis.ResizeObserver) {
    globalThis.ResizeObserver = vi.fn().mockImplementation(() => ({
      observe: vi.fn(),
      unobserve: vi.fn(),
      disconnect: vi.fn(),
    }));
  }
};

setupBrowserPolyfills();

// Add custom matchers (preparation for future jest-dom integration)
expect.extend({
  // Example: custom matcher
  toBeValidElement(received: any) {
    const pass = received && typeof received === "object" && received.nodeType === 1;
    return {
      pass,
      message: () =>
        pass
          ? `expected ${received} not to be a valid DOM element`
          : `expected ${received} to be a valid DOM element`,
    };
  },
});

export {};
