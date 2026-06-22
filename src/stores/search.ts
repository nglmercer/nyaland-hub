import { signal, computed } from "@preact/signals";
import type { Torrent, TorrentDetail, SearchParams, SearchResult } from "../types";

export const searchQuery = signal("");
export const searchResults = signal<Torrent[]>([]);
export const searchLoading = signal(false);
export const searchPage = signal(1);
export const searchTotalPages = signal<number | null>(null);
export const searchCategory = signal("all");
export const searchFilter = signal("no filter");
export const searchSort = signal("date");
export const searchOrder = signal("desc");
export const selectedTorrent = signal<TorrentDetail | null>(null);
export const detailLoading = signal(false);

export const hasResults = computed(() => searchResults.value.length > 0);

export async function performSearch() {
  const { invoke } = await import("@tauri-apps/api/core");

  searchLoading.value = true;
  try {
    const params: SearchParams = {
      query: searchQuery.value,
      page: searchPage.value,
      category: searchCategory.value,
      filter: searchFilter.value,
      sort: searchSort.value,
      order: searchOrder.value,
    };
    const raw = await invoke<string>("search", { params });
    const result: SearchResult = JSON.parse(raw);
    searchResults.value = result.data;
    searchTotalPages.value = result.total_page;
  } catch (e) {
    console.error("Search failed:", e);
    searchResults.value = [];
  } finally {
    searchLoading.value = false;
  }
}

export async function viewTorrent(id: number) {
  const { invoke } = await import("@tauri-apps/api/core");

  detailLoading.value = true;
  try {
    const raw = await invoke<string>("view_torrent", { id });
    selectedTorrent.value = JSON.parse(raw);
  } catch (e) {
    console.error("View failed:", e);
    selectedTorrent.value = null;
  } finally {
    detailLoading.value = false;
  }
}
