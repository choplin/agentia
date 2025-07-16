import { writable } from "svelte/store";
import { browser } from "$app/environment";

export type Theme = "light" | "dark" | "system";

// Create theme store with localStorage persistence
function createThemeStore() {
  // Get initial theme from localStorage or default to system
  const storedTheme = browser ? (localStorage.getItem("theme") as Theme) : null;
  const initialTheme: Theme = storedTheme || "system";

  const { subscribe, set, update } = writable<Theme>(initialTheme);

  return {
    subscribe,
    set: (theme: Theme) => {
      set(theme);
      if (browser) {
        localStorage.setItem("theme", theme);
        applyTheme(theme);
      }
    },
    update,
    init: () => {
      if (browser) {
        applyTheme(initialTheme);
      }
    },
  };
}

// Apply theme to document
function applyTheme(theme: Theme) {
  const root = document.documentElement;

  // Remove existing theme classes
  root.classList.remove("light", "dark");

  if (theme === "system") {
    // Use system preference
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    root.classList.add(prefersDark ? "dark" : "light");
  } else {
    // Use explicit theme
    root.classList.add(theme);
  }
}

// Listen for system theme changes
if (browser) {
  window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
    const currentTheme = localStorage.getItem("theme") as Theme;
    if (currentTheme === "system" || !currentTheme) {
      applyTheme("system");
    }
  });
}

export const theme = createThemeStore();
