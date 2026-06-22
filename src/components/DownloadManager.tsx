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
import { PauseIcon, PlayIcon, TrashIcon } from "./icons";

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
    case "Downloading": return "state-downloading";
    case "Paused": return "state-paused";
    case "Finished": return "state-finished";
    case "Queued": return "state-queued";
    case "Error": return "state-error";
    default: return "";
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
              h("div", { class: "download-progress" },
                h("div", {
                  class: "progress-bar",
                  style: { width: `${Math.round(d.progress * 100)}%` },
                }),
              ),
              h("div", { class: "download-actions" },
                d.state === "Downloading"
                  ? h("button", {
                      class: "btn btn-small",
                      onClick: () => pauseDownload(d.hash),
                    },
                      h(PauseIcon, { size: 14 }),
                      tx.btnPause,
                    )
                  : h("button", {
                      class: "btn btn-small",
                      onClick: () => resumeDownload(d.hash),
                    },
                      h(PlayIcon, { size: 14 }),
                      tx.btnResume,
                    ),
                h("button", {
                  class: "btn btn-small btn-danger",
                  onClick: () => removeDownload(d.hash, false),
                },
                  h(TrashIcon, { size: 14 }),
                ),
              ),
            )
          ),
          ...completedDownloads.value.map((d) =>
            h("div", { key: d.hash, class: "download-item state-finished" },
              h("div", { class: "download-info" },
                h("div", { class: "download-name" }, d.name),
                h("div", { class: "download-meta" },
                  h("span", null, `${formatBytes(d.total_size)} - ${tx.downloadComplete}`),
                ),
              ),
              h("div", { class: "download-actions" },
                h("button", {
                  class: "btn btn-small btn-danger",
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
