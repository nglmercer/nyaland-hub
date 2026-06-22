import { h } from "preact";
import { useSignal } from "@preact/signals";
import {
  selectedTorrent,
  detailLoading,
  selectedTorrent as detail,
} from "../stores/search";
import { addDownload } from "../stores/downloads";
import { settings } from "../stores/settings";
import { t } from "../i18n";
import { CloseIcon, MagnetIcon } from "./icons";

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
        h("span", null, `${torrent.category} / ${torrent.sub_category}`),
      ),
      h("div", { class: "meta-item" },
        h("span", { class: "meta-label" }, tx.detailSize),
        h("span", null, torrent.size),
      ),
      h("div", { class: "meta-item" },
        h("span", { class: "meta-label" }, tx.detailDate),
        h("span", null, torrent.date),
      ),
      h("div", { class: "meta-item" },
        h("span", { class: "meta-label" }, tx.detailSeeders),
        h("span", { class: "text-success" }, String(torrent.seeders)),
      ),
      h("div", { class: "meta-item" },
        h("span", { class: "meta-label" }, tx.detailLeechers),
        h("span", null, String(torrent.leechers)),
      ),
      h("div", { class: "meta-item" },
        h("span", { class: "meta-label" }, tx.detailDownloads),
        h("span", null, String(torrent.downloads)),
      ),
      h("div", { class: "meta-item" },
        h("span", { class: "meta-label" }, tx.detailSubmitter),
        h("span", null, torrent.submitter || tx.detailAnonymous),
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
          h("pre", { class: "description-text" }, torrent.description),
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