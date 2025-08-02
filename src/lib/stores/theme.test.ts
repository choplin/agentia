import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import { get } from "svelte/store";

// Mock $app/environment
vi.mock("$app/environment", () => ({
  browser: true,
}));

// Mock localStorage
const localStorageMock = {
  store: {} as Record<string, string>,
  getItem: vi.fn((_key: string): string | null => null),
  setItem: vi.fn((key: string, value: string) => {
    localStorageMock.store[key] = value;
  }),
  removeItem: vi.fn((key: string) => {
    delete localStorageMock.store[key];
  }),
  clear: vi.fn(() => {
    localStorageMock.store = {};
  }),
};

Object.defineProperty(window, "localStorage", {
  value: localStorageMock,
});

// Mock matchMedia
const matchMediaMock = vi.fn((query: string) => ({
  matches: false,
  media: query,
  onchange: null,
  addEventListener: vi.fn(),
  removeEventListener: vi.fn(),
  dispatchEvent: vi.fn(),
}));

Object.defineProperty(window, "matchMedia", {
  value: matchMediaMock,
  writable: true,
});

// Mock document.documentElement.classList
const mockClassList = {
  add: vi.fn(),
  remove: vi.fn(),
  contains: vi.fn(),
};

Object.defineProperty(document.documentElement, "classList", {
  value: mockClassList,
  writable: true,
});

describe("theme store", () => {
  let theme: any;

  beforeEach(() => {
    // Clear all mocks
    vi.clearAllMocks();
    localStorageMock.clear();
    mockClassList.add.mockClear();
    mockClassList.remove.mockClear();

    // Reset modules to ensure fresh store creation
    vi.resetModules();

    // Set default matchMedia behavior
    matchMediaMock.mockReturnValue({
      matches: false,
      media: "(prefers-color-scheme: dark)",
      onchange: null,
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn(),
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  describe("initialization", () => {
    it("should initialize with system theme by default", async () => {
      const { theme } = await import("./theme");
      expect(get(theme)).toBe("system");
    });

    it("should initialize with stored theme from localStorage", async () => {
      // Set localStorage before importing
      localStorageMock.store = { theme: "dark" };
      localStorageMock.getItem = vi.fn(() => "dark");

      const { theme } = await import("./theme");
      expect(get(theme)).toBe("dark");
    });

    it("should apply theme on init", async () => {
      const { theme } = await import("./theme");
      theme.init();

      expect(mockClassList.remove).toHaveBeenCalledWith("light", "dark");
      expect(mockClassList.add).toHaveBeenCalledWith("light");
    });

    it("should apply dark theme when system prefers dark", async () => {
      matchMediaMock.mockReturnValue({
        matches: true,
        media: "(prefers-color-scheme: dark)",
        onchange: null,
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      });

      const { theme } = await import("./theme");
      theme.init();

      expect(mockClassList.add).toHaveBeenCalledWith("dark");
    });
  });

  describe("set theme", () => {
    beforeEach(async () => {
      const module = await import("./theme");
      theme = module.theme;
    });

    it("should set light theme", () => {
      theme.set("light");

      expect(get(theme)).toBe("light");
      expect(localStorageMock.setItem).toHaveBeenCalledWith("theme", "light");
      expect(mockClassList.remove).toHaveBeenCalledWith("light", "dark");
      expect(mockClassList.add).toHaveBeenCalledWith("light");
    });

    it("should set dark theme", () => {
      theme.set("dark");

      expect(get(theme)).toBe("dark");
      expect(localStorageMock.setItem).toHaveBeenCalledWith("theme", "dark");
      expect(mockClassList.remove).toHaveBeenCalledWith("light", "dark");
      expect(mockClassList.add).toHaveBeenCalledWith("dark");
    });

    it("should set system theme and apply based on preference", () => {
      theme.set("system");

      expect(get(theme)).toBe("system");
      expect(localStorageMock.setItem).toHaveBeenCalledWith("theme", "system");
      expect(mockClassList.add).toHaveBeenCalledWith("light");
    });

    it("should apply dark when system theme with dark preference", () => {
      matchMediaMock.mockReturnValue({
        matches: true,
        media: "(prefers-color-scheme: dark)",
        onchange: null,
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      });

      theme.set("system");
      expect(mockClassList.add).toHaveBeenCalledWith("dark");
    });
  });

  describe("system theme change listener", () => {
    it("should register listener for system theme changes", async () => {
      const addEventListenerMock = vi.fn();
      matchMediaMock.mockReturnValue({
        matches: false,
        media: "(prefers-color-scheme: dark)",
        onchange: null,
        addEventListener: addEventListenerMock,
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      });

      await import("./theme");

      expect(matchMediaMock).toHaveBeenCalledWith("(prefers-color-scheme: dark)");
      expect(addEventListenerMock).toHaveBeenCalledWith("change", expect.any(Function));
    });

    it("should update theme when system preference changes", async () => {
      let changeCallback: (() => void) | undefined;
      const addEventListenerMock = vi.fn((event: string, callback: any) => {
        if (event === "change") {
          changeCallback = callback as () => void;
        }
      });

      matchMediaMock.mockReturnValue({
        matches: false,
        media: "(prefers-color-scheme: dark)",
        onchange: null,
        addEventListener: addEventListenerMock,
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      });

      const { theme } = await import("./theme");
      theme.set("system");

      // Clear previous calls
      mockClassList.add.mockClear();
      mockClassList.remove.mockClear();

      // Simulate system theme change to dark
      matchMediaMock.mockReturnValue({
        matches: true,
        media: "(prefers-color-scheme: dark)",
        onchange: null,
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      });

      // Trigger the change event
      expect(changeCallback).not.toBeNull();
      changeCallback?.();

      expect(mockClassList.remove).toHaveBeenCalledWith("light", "dark");
      expect(mockClassList.add).toHaveBeenCalledWith("dark");
    });

    it("should not update theme when not in system mode", async () => {
      let changeCallback: (() => void) | undefined;
      const addEventListenerMock = vi.fn((event: string, callback: any) => {
        if (event === "change") {
          changeCallback = callback as () => void;
        }
      });

      matchMediaMock.mockReturnValue({
        matches: false,
        media: "(prefers-color-scheme: dark)",
        onchange: null,
        addEventListener: addEventListenerMock,
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      });

      const { theme } = await import("./theme");

      // Set theme to light and ensure localStorage reflects it
      theme.set("light");
      localStorageMock.getItem = vi.fn(() => "light");

      // Clear previous calls
      mockClassList.add.mockClear();
      mockClassList.remove.mockClear();

      // Trigger the change event
      expect(changeCallback).not.toBeNull();
      changeCallback?.();

      // Should not apply theme changes when not in system mode
      expect(mockClassList.add).not.toHaveBeenCalled();
    });
  });

  describe("browser environment", () => {
    it("should handle non-browser environment", async () => {
      // Mock non-browser environment
      vi.doMock("$app/environment", () => ({
        browser: false,
      }));

      const { theme } = await import("./theme");

      // Should still work but not interact with DOM/localStorage
      expect(get(theme)).toBe("system");

      theme.set("dark");
      expect(localStorageMock.setItem).not.toHaveBeenCalled();
      expect(mockClassList.add).not.toHaveBeenCalled();
    });
  });
});
