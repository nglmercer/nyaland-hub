import { h } from "preact";
import { useSignal } from "@preact/signals";
import { SearchBar } from "./components/SearchBar";
import { TorrentList } from "./components/TorrentList";
import { TorrentDetail } from "./components/TorrentDetail";
import { DownloadManager } from "./components/DownloadManager";
import { Settings } from "./components/Settings";
import { SearchIcon, DownloadIcon, SettingsIcon } from "./components/icons";
import { startPolling, stopPolling } from "./stores/downloads";
import { selectedTorrent } from "./stores/search";
import { loadSettings } from "./stores/settings";
import { t } from "./i18n";

type Tab = "search" | "downloads" | "settings";

export function App() {
  const activeTab = useSignal<Tab>("search");

  const switchTab = (tab: Tab) => {
    activeTab.value = tab;
    if (tab === "downloads") {
      startPolling();
    } else {
      stopPolling();
    }
  };

  loadSettings();

  return h("div", { class: "app" },
    h("header", { class: "app-header" },
      h("h1", { class: "app-title" }, "Nyaland"),
      h("nav", { class: "app-nav" },
        h("button", {
          class: `nav-btn ${activeTab.value === "search" ? "active" : ""}`,
          onClick: () => switchTab("search"),
        },
          h(SearchIcon, { size: 16 }),
          h("span", null, t.value.navSearch),
        ),
        h("button", {
          class: `nav-btn ${activeTab.value === "downloads" ? "active" : ""}`,
          onClick: () => switchTab("downloads"),
        },
          h(DownloadIcon, { size: 16 }),
          h("span", null, t.value.navDownloads),
        ),
        h("button", {
          class: `nav-btn ${activeTab.value === "settings" ? "active" : ""}`,
          onClick: () => switchTab("settings"),
        },
          h(SettingsIcon, { size: 16 }),
          h("span", null, t.value.navSettings),
        ),
      ),
    ),
    h("main", { class: "app-content" },
      activeTab.value === "search" && h("div", { class: "search-layout" },
        h("div", { class: "search-panel" },
          h(SearchBar, null),
          h(TorrentList, null),
        ),
        selectedTorrent.value !== null && h("div", { class: "detail-panel" },
          h(TorrentDetail, null),
        ),
      ),
      activeTab.value === "downloads" && h(DownloadManager, null),
      activeTab.value === "settings" && h(Settings, null),
    ),
  );
}
