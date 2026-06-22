import { h } from "preact";
import { useSignal } from "@preact/signals";
import { marked } from "marked";
import {
  selectedTorrent,
  detailLoading,
  selectedTorrent as detail,
} from "../stores/search";
import { addDownload } from "../stores/downloads";
import { settings } from "../stores/settings";
import { t } from "../i18n";
import { CloseIcon, MagnetIcon } from "./icons";

marked.setOptions({
  breaks: true,
  gfm: true,
});

function renderMarkdown(text: string): string {
  return marked.parse(text) as string;
}

export function TorrentDetail() {
  const addingDownload = useSignal(false);

  const tx = t.value;

  if (detailLoading.value) {
    return h("div", { class: "drawer-content loading" },
      h("p", null, tx.loadingDetails),
    );
  }

  const torrent = detail.value;
  if (!torrent) {
    return h("div", { class: "drawer-content empty" },
      h("p", null, tx.selectTorrent),
    );
  }

  const handleDownload = async () => {
    addingDownload.value = true;
    try {
      await addDownload(torrent.magnet, settings.value.save_path);
      selectedTorrent.value = null;
    } finally {
      addingDownload.value = false;
    }
  };

  return h("div", { class: "drawer-content" },
    h("div", { class: "drawer-header" },
      h("button", {
        class: "btn btn-sm btn-close",
        onClick: () => { selectedTorrent.value = null; },
      }, h(CloseIcon, { size: 14 })),
      h("h2", null, torrent.title),
    ),
    h("div", { class: "detail-meta" },
      h("div", { class: "meta-item" },
        h("span", { class: "meta-label" }, tx.detailCategory),
        h("span", { class: "meta-value badge" }, `${torrent.category} / ${torrent.sub_category}`),
      ),
      h("div", { class: "meta-item" },
        h("span", { class: "meta-label" }, tx.detailSize),
        h("span", { class: "meta-value" }, torrent.size),
      ),
      h("div", { class: "meta-item" },
        h("span", { class: "meta-label" }, tx.detailDate),
        h("span", { class: "meta-value" }, torrent.date),
      ),
      h("div", { class: "meta-grid" },
        h("div", { class: "meta-stat" },
          h("span", { class: "meta-stat-value text-success" }, String(torrent.seeders)),
          h("span", { class: "meta-stat-label" }, tx.detailSeeders),
        ),
        h("div", { class: "meta-stat" },
          h("span", { class: "meta-stat-value text-error" }, String(torrent.leechers)),
          h("span", { class: "meta-stat-label" }, tx.detailLeechers),
        ),
        h("div", { class: "meta-stat" },
          h("span", { class: "meta-stat-value" }, String(torrent.downloads)),
          h("span", { class: "meta-stat-label" }, tx.detailDownloads),
        ),
      ),
      h("div", { class: "meta-item" },
        h("span", { class: "meta-label" }, tx.detailSubmitter),
        h("span", { class: "meta-value" }, torrent.submitter || tx.detailAnonymous),
      ),
    ),
    torrent.files.length > 0
      ? h("div", { class: "detail-files" },
          h("h3", null, tx.detailFiles),
          h("ul", null,
            torrent.files.map((f, i) =>
              h("li", { key: i, class: "file-item" },
                h("span", { class: "file-name" }, f.name),
                h("span", { class: "file-size" }, f.size),
              )
            ),
          ),
        )
      : null,
    torrent.description
      ? h("div", { class: "detail-description" },
          h("h3", null, tx.detailDescription),
          h("div", {
            class: "description-text markdown-body",
            dangerouslySetInnerHTML: { __html: renderMarkdown(torrent.description) },
          }),
        )
      : null,
    h("div", { class: "detail-actions" },
      h("button", {
        class: "btn btn-primary btn-download-start",
        onClick: handleDownload,
        disabled: addingDownload.value,
      }, addingDownload.value ? tx.btnAdding : tx.btnStartDownload),
      h("a", {
        class: "btn btn-secondary btn-magnet",
        href: torrent.magnet,
        target: "_blank",
      },
        h(MagnetIcon, { size: 14 }),
        tx.btnCopyMagnet,
      ),
    ),
  );
}
