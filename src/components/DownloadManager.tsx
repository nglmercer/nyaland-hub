import { h } from "preact";
import {
  downloads,
  activeDownloads,
  totalDownloadSpeed,
  totalUploadSpeed,
  pauseDownload,
  resumeDownload,
  removeDownload,
  completedDownloads,
} from "../stores/downloads";
import { t } from "../i18n";
import { PauseIcon, PlayIcon, TrashIcon, FolderOpenIcon } from "./icons";

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
}

function formatSpeed(bytesPerSec: number): string {
  return formatBytes(bytesPerSec) + "/s";
}

function stateColor(state: string): string {
  switch (state) {
    case "Downloading": return "is-downloading";
    case "Paused": return "is-paused";
    case "Finished": return "is-finished";
    case "Queued": return "is-queued";
    case "Error": return "is-error";
    default: return "";
  }
}

async function playFile(path: string) {
  const { invoke } = await import("@tauri-apps/api/core");
  try {
    const files: string[] = await invoke("detect_media_files", { path });
    if (files.length === 0) {
      // Try recursive detection for nested torrent structures
      const allFiles: string[] = await invoke("detect_media_files_recursive", { path });
      if (allFiles.length > 0) {
        await invoke("play_file", { path: allFiles[0] });
      }
      return;
    }

    await invoke("play_file", { path: files[0] });
  } catch (e) {
    console.error("Play failed:", e);
  }
}

async function openFolder(path: string) {
  const { invoke } = await import("@tauri-apps/api/core");
  try {
    await invoke("open_folder", { path });
  } catch (e) {
    console.error("Open folder failed:", e);
  }
}

export function DownloadManager() {
  const tx = t.value;

  return h("div", { class: "download-manager" },
    h("div", { class: "download-stats" },
      h("span", null, `${tx.downloadActive} ${activeDownloads.value.length}`),
      h("span", null, `${tx.downloadDown} ${formatSpeed(totalDownloadSpeed.value)}`),
      h("span", null, `${tx.downloadUp} ${formatSpeed(totalUploadSpeed.value)}`),
    ),
    activeDownloads.value.length > 0 || completedDownloads.value.length > 0
      ? h("div", { class: "download-list" },
          ...activeDownloads.value.map((d) =>
            h("div", { key: d.hash, class: `download-item ${stateColor(d.state)}` },
              h("div", { class: "download-info" },
                h("div", { class: "download-name" }, d.name),
                h("div", { class: "download-meta" },
                  h("span", null, `${formatBytes(d.downloaded)} / ${formatBytes(d.total_size)}`),
                  h("span", null, `${Math.round(d.progress * 100)}%`),
                  h("span", null, `${d.num_peers} ${tx.downloadPeers}`),
                  h("span", null, `${tx.downloadDown} ${formatSpeed(d.download_rate)}`),
                  h("span", null, `${tx.downloadUp} ${formatSpeed(d.upload_rate)}`),
                ),
              ),
              h("div", { class: "progress" },
                h("div", {
                  class: "progress-bar",
                  style: { width: `${Math.round(d.progress * 100)}%` },
                }),
              ),
              h("div", { class: "download-actions" },
                d.state === "Downloading"
                  ? h("button", {
                      class: "btn btn-sm",
                      onClick: () => pauseDownload(d.hash),
                    },
                      h(PauseIcon, { size: 14 }),
                      tx.btnPause,
                    )
                  : h("button", {
                      class: "btn btn-sm",
                      onClick: () => resumeDownload(d.hash),
                    },
                      h(PlayIcon, { size: 14 }),
                      tx.btnResume,
                    ),
                h("button", {
                  class: "btn btn-sm btn-error",
                  onClick: () => removeDownload(d.hash, false),
                },
                  h(TrashIcon, { size: 14 }),
                ),
              ),
            )
          ),
          ...completedDownloads.value.map((d) =>
            h("div", { key: d.hash, class: "download-item is-finished" },
              h("div", { class: "download-info" },
                h("div", { class: "download-name" }, d.name),
                h("div", { class: "download-meta" },
                  h("span", null, `${formatBytes(d.total_size)} - ${tx.downloadComplete}`),
                  h("span", null, d.save_path),
                ),
              ),
              h("div", { class: "download-actions" },
                h("button", {
                  class: "btn btn-sm btn-primary",
                  onClick: () => playFile(d.save_path),
                },
                  h(PlayIcon, { size: 14 }),
                  tx.btnPlay,
                ),
                h("button", {
                  class: "btn btn-sm",
                  onClick: () => openFolder(d.save_path),
                },
                  h(FolderOpenIcon, { size: 14 }),
                  tx.btnOpenFolder,
                ),
                h("button", {
                  class: "btn btn-sm btn-error",
                  onClick: () => removeDownload(d.hash, false),
                },
                  h(TrashIcon, { size: 14 }),
                  tx.btnRemove,
                ),
              ),
            )
          ),
        )
      : h("div", { class: "empty-state" },
          h("p", null, tx.downloadEmpty),
        ),
  );
}