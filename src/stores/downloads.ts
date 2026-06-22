import { signal, computed } from "@preact/signals";
import type { DownloadStatus } from "../types";

export const downloads = signal<DownloadStatus[]>([]);
export const downloadsLoading = signal(false);

export const activeDownloads = computed(() =>
  downloads.value.filter((d) => d.state === "Downloading" || d.state === "Queued")
);

export const completedDownloads = computed(() =>
  downloads.value.filter((d) => d.state === "Finished")
);

export const totalDownloadSpeed = computed(() =>
  activeDownloads.value.reduce((sum, d) => sum + d.download_rate, 0)
);

export const totalUploadSpeed = computed(() =>
  activeDownloads.value.reduce((sum, d) => sum + d.upload_rate, 0)
);

let pollInterval: number | null = null;

export async function fetchDownloads() {
  const { invoke } = await import("@tauri-apps/api/core");

  try {
    const raw = await invoke<string>("get_downloads");
    downloads.value = JSON.parse(raw);
  } catch (e) {
    console.error("Fetch downloads failed:", e);
  }
}

export async function addDownload(magnet: string, savePath: string) {
  const { invoke } = await import("@tauri-apps/api/core");

  try {
    await invoke<string>("add_download", {
      params: { magnet, save_path: savePath },
    });
    await fetchDownloads();
  } catch (e) {
    console.error("Add download failed:", e);
  }
}

export async function pauseDownload(hash: string) {
  const { invoke } = await import("@tauri-apps/api/core");

  try {
    await invoke("pause_download", { params: { hash } });
    await fetchDownloads();
  } catch (e) {
    console.error("Pause failed:", e);
  }
}

export async function resumeDownload(hash: string) {
  const { invoke } = await import("@tauri-apps/api/core");

  try {
    await invoke("resume_download", { params: { hash } });
    await fetchDownloads();
  } catch (e) {
    console.error("Resume failed:", e);
  }
}

export async function removeDownload(hash: string, deleteFiles: boolean) {
  const { invoke } = await import("@tauri-apps/api/core");

  try {
    await invoke("remove_download", {
      params: { hash, delete_files: deleteFiles },
    });
    await fetchDownloads();
  } catch (e) {
    console.error("Remove failed:", e);
  }
}

export function startPolling() {
  if (pollInterval) return;
  fetchDownloads();
  pollInterval = window.setInterval(fetchDownloads, 1500);
}

export function stopPolling() {
  if (pollInterval) {
    clearInterval(pollInterval);
    pollInterval = null;
  }
}
