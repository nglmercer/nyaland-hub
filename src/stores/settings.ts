import { signal } from "@preact/signals";
import type { AppSettings } from "../types";

export const settings = signal<AppSettings>({
  save_path: "~/Downloads/Nyaland",
  nyaa_base_url: "https://nyaa.si",
  max_download_speed: 0,
  max_upload_speed: 0,
  max_connections: 200,
  max_active_downloads: 5,
  start_on_launch: true,
});

export const settingsLoading = signal(false);

export async function loadSettings() {
  const { invoke } = await import("@tauri-apps/api/core");

  settingsLoading.value = true;
  try {
    const raw = await invoke<string>("get_settings");
    settings.value = JSON.parse(raw);
  } catch (e) {
    console.error("Load settings failed:", e);
  } finally {
    settingsLoading.value = false;
  }
}

export async function saveSettings(newSettings: AppSettings) {
  const { invoke } = await import("@tauri-apps/api/core");

  try {
    await invoke("save_settings", { settings: newSettings });
    settings.value = newSettings;
  } catch (e) {
    console.error("Save settings failed:", e);
  }
}
