import { h } from "preact";
import {
  searchResults,
  searchPage,
  searchTotalPages,
  performSearch,
  viewTorrent,
  hasResults,
} from "../stores/search";
import { t } from "../i18n";
import { ChevronLeftIcon, ChevronRightIcon } from "./icons";

function formatDate(dateStr: string): string {
  try {
    const d = new Date(dateStr);
    return d.toLocaleDateString();
  } catch {
    return dateStr;
  }
}

function seederClass(seeders: number): string {
  if (seeders > 50) return "text-success";
  if (seeders > 10) return "text-warning";
  return "text-secondary";
}

export function TorrentList() {
  const goToPage = (page: number) => {
    searchPage.value = page;
    performSearch();
  };

  const tx = t.value;

  return h("div", { class: "torrent-list" },
    hasResults.value
      ? h("div", { class: "table-wrapper" },
          h("table", { class: "table table-hover table-pin-rows" },
            h("thead", null,
              h("tr", null,
                h("th", null, tx.colName),
                h("th", null, tx.colSize),
                h("th", null, tx.colSeeders),
                h("th", null, tx.colLeechers),
                h("th", null, tx.colDownloads),
                h("th", null, tx.colDate),
                h("th", null, ""),
              ),
            ),
            h("tbody", null,
              searchResults.value.map((t) =>
                h("tr", {
                  key: t.id,
                  class: "table-row",
                  onClick: () => viewTorrent(t.id),
                },
                  h("td", { class: "torrent-name" },
                    h("span", { class: "badge" }, t.category),
                    t.name,
                  ),
                  h("td", { class: "torrent-size" }, t.size),
                  h("td", { class: `torrent-seeders ${seederClass(t.seeders)}` },
                    String(t.seeders),
                  ),
                  h("td", { class: "torrent-leechers" }, String(t.leechers)),
                  h("td", { class: "torrent-downloads" }, String(t.downloads)),
                  h("td", { class: "torrent-date" }, formatDate(t.date)),
                  h("td", null,
                    h("button", {
                      class: "btn btn-sm btn-primary",
                      onClick: (e: Event) => {
                        e.stopPropagation();
                        viewTorrent(t.id);
                      },
                    }, tx.btnView),
                  ),
                )
              ),
            ),
          ),
          h("div", { class: "pagination" },
            searchPage.value > 1 &&
              h("button", {
                class: "btn btn-sm",
                onClick: () => goToPage(searchPage.value - 1),
              },
                h(ChevronLeftIcon, { size: 14 }),
                tx.paginationPrev,
              ),
            h("span", { class: "page-info" },
              `${tx.paginationPage} ${searchPage.value}${searchTotalPages.value ? ` ${tx.paginationOf} ${searchTotalPages.value}` : ""}`,
            ),
            searchTotalPages.value && searchPage.value < searchTotalPages.value
              ? h("button", {
                  class: "btn btn-sm",
                  onClick: () => goToPage(searchPage.value + 1),
                },
                  tx.paginationNext,
                  h(ChevronRightIcon, { size: 14 }),
                )
              : null,
          ),
        )
      : h("div", { class: "empty-state" },
          h("p", null, tx.emptySearch),
        ),
  );
}