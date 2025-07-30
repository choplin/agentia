/**
 * Vitest global setup file
 * This file is executed before each test file
 */

import { expect, vi, afterEach } from "vitest";
import { clearMocks } from "@tauri-apps/api/mocks";
import "@testing-library/jest-dom";

// Global configuration
declare global {
  var __TEST_MODE__: boolean;
}

// Set test mode flag
globalThis.__TEST_MODE__ = true;

// Provide mock reset functionality between tests
afterEach(() => {
  // Clear Tauri mocks
  clearMocks();

  // Clear all DOM modifications
  document.body.innerHTML = "";
  document.head.innerHTML = "";

  // Clear all timers
  vi.clearAllTimers();

  // Reset all mocks
  vi.resetAllMocks();

  // Clear any custom properties added to window/global
  if (typeof window !== "undefined") {
    const windowKeys = Object.keys(window);
    windowKeys.forEach((key) => {
      if (key.startsWith("__test_") || key.startsWith("test_")) {
        // Use bracket notation with a type assertion on the result
        const windowObj = window as unknown as Record<string, unknown>;
        delete windowObj[key];
      }
    });
  }
});

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
