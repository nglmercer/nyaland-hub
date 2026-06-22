import { signal, computed } from "@preact/signals";

export type Theme = "dark" | "light";

const STORAGE_KEY = "nyaland-theme";

function getInitialTheme(): Theme {
  if (typeof window !== "undefined") {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored === "dark" || stored === "light") return stored;
  }
  return "dark";
}

export const theme = signal<Theme>(getInitialTheme());

export const isDark = computed(() => theme.value === "dark");

export function toggleTheme() {
  theme.value = theme.value === "dark" ? "light" : "dark";
  localStorage.setItem(STORAGE_KEY, theme.value);
  applyTheme(theme.value);
}

export function setTheme(t: Theme) {
  theme.value = t;
  localStorage.setItem(STORAGE_KEY, t);
  applyTheme(t);
}

function applyTheme(t: Theme) {
  document.documentElement.setAttribute("data-theme", t);
}

// Apply on load
if (typeof window !== "undefined") {
  applyTheme(theme.value);
}
