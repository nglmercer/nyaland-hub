import { h } from "preact";
import { useEffect } from "preact/hooks";
import { useSignal } from "@preact/signals";
import { SearchBar } from "./components/SearchBar";
import { TorrentList } from "./components/TorrentList";
import { TorrentDetail } from "./components/TorrentDetail";
import { DownloadManager } from "./components/DownloadManager";
import { Settings } from "./components/Settings";
import { SearchIcon, DownloadIcon, SettingsIcon, SunIcon, MoonIcon, HamburgerIcon, CloseIcon } from "./components/icons";
import { startPolling, stopPolling } from "./stores/downloads";
import { selectedTorrent, performSearch, searchQuery, searchLoading } from "./stores/search";
import { loadSettings } from "./stores/settings";
import { t } from "./i18n";
import { theme, toggleTheme, isDark } from "./stores/theme";

type Tab = "search" | "downloads" | "settings";

export function App() {
  const activeTab = useSignal<Tab>("search");
  const sidebarOpen = useSignal(false);
  const localQuery = useSignal(searchQuery.value);

  useEffect(() => {
    performSearch();
  }, []);

  const switchTab = (tab: Tab) => {
    activeTab.value = tab;
    sidebarOpen.value = false;
    if (tab === "downloads") {
      startPolling();
    } else {
      stopPolling();
    }
  };

  const handleSearchSubmit = (e: Event) => {
    e.preventDefault();
    searchQuery.value = localQuery.value;
    performSearch();
  };

  loadSettings();

  const tx = t.value;

  return h("div", { class: "app" },
    h("header", { class: "navbar" },
      h("button", {
        class: "hamburger-btn",
        onClick: () => { sidebarOpen.value = !sidebarOpen.value; },
      },
        sidebarOpen.value ? h(CloseIcon, { size: 20 }) : h(HamburgerIcon, { size: 20 }),
      ),
      h("h1", { class: "navbar-brand" }, "Nyaland"),
      h("form", { class: "navbar-search", onSubmit: handleSearchSubmit },
        h("input", {
          type: "text",
          class: "input",
          placeholder: tx.searchPlaceholder,
          value: localQuery.value,
          onInput: (e: Event) => {
            localQuery.value = (e.target as HTMLInputElement).value;
          },
        }),
        h("button", {
          type: "submit",
          class: "btn btn-primary btn-search",
          disabled: searchLoading.value,
        },
          h(SearchIcon, { size: 16 }),
        ),
      ),
      h("nav", { class: "navbar-menu" },
        h("button", {
          class: `nav-item ${activeTab.value === "search" ? "active" : ""}`,
          onClick: () => switchTab("search"),
        },
          h(SearchIcon, { size: 16 }),
          h("span", null, tx.navSearch),
        ),
        h("button", {
          class: `nav-item ${activeTab.value === "downloads" ? "active" : ""}`,
          onClick: () => switchTab("downloads"),
        },
          h(DownloadIcon, { size: 16 }),
          h("span", null, tx.navDownloads),
        ),
        h("button", {
          class: `nav-item ${activeTab.value === "settings" ? "active" : ""}`,
          onClick: () => switchTab("settings"),
        },
          h(SettingsIcon, { size: 16 }),
          h("span", null, tx.navSettings),
        ),
        h("button", {
          class: "theme-toggle",
          onClick: toggleTheme,
          title: isDark.value ? tx.themeLight : tx.themeDark,
        },
          isDark.value ? h(SunIcon, { size: 16 }) : h(MoonIcon, { size: 16 }),
        ),
      ),
    ),
    h("div", { class: "mobile-search" },
      h("form", { class: "mobile-search-form", onSubmit: handleSearchSubmit },
        h("input", {
          type: "text",
          class: "input",
          placeholder: tx.searchPlaceholder,
          value: localQuery.value,
          onInput: (e: Event) => {
            localQuery.value = (e.target as HTMLInputElement).value;
          },
        }),
        h("button", {
          type: "submit",
          class: "btn btn-primary btn-search",
          disabled: searchLoading.value,
        },
          h(SearchIcon, { size: 16 }),
        ),
      ),
    ),
    h("div", {
      class: `sidebar-overlay${sidebarOpen.value ? " open" : ""}`,
      onClick: () => { sidebarOpen.value = false; },
    }),
    h("aside", { class: `sidebar${sidebarOpen.value ? " open" : ""}` },
      h("div", { class: "sidebar-header" },
        h("h2", null, "Nyaland"),
      ),
      h("nav", { class: "sidebar-nav" },
        h("button", {
          class: `sidebar-item ${activeTab.value === "search" ? "active" : ""}`,
          onClick: () => switchTab("search"),
        },
          h(SearchIcon, { size: 18 }),
          h("span", null, tx.navSearch),
        ),
        h("button", {
          class: `sidebar-item ${activeTab.value === "downloads" ? "active" : ""}`,
          onClick: () => switchTab("downloads"),
        },
          h(DownloadIcon, { size: 18 }),
          h("span", null, tx.navDownloads),
        ),
        h("button", {
          class: `sidebar-item ${activeTab.value === "settings" ? "active" : ""}`,
          onClick: () => switchTab("settings"),
        },
          h(SettingsIcon, { size: 18 }),
          h("span", null, tx.navSettings),
        ),
      ),
      h("div", { class: "sidebar-footer" },
        h("button", {
          class: "sidebar-item",
          onClick: toggleTheme,
        },
          isDark.value ? h(SunIcon, { size: 18 }) : h(MoonIcon, { size: 18 }),
          h("span", null, isDark.value ? tx.themeLight : tx.themeDark),
        ),
      ),
    ),
    h("main", { class: "app-content" },
      activeTab.value === "search" && h("div", { class: "search-layout" },
        h("div", { class: "search-panel" },
          h(SearchBar, null),
          h(TorrentList, null),
        ),
        h("div", { class: `drawer${selectedTorrent.value !== null ? " drawer-open" : ""}` },
          h(TorrentDetail, null),
        ),
      ),
      activeTab.value === "downloads" && h(DownloadManager, null),
      activeTab.value === "settings" && h(Settings, null),
    ),
  );
}
