import { h } from "preact";
import { useSignal } from "@preact/signals";
import {
  searchQuery,
  searchCategory,
  searchFilter,
  searchSort,
  searchOrder,
  performSearch,
  searchLoading,
} from "../stores/search";
import { FilterIcon, ChevronDownIcon } from "./icons";
import { t } from "../i18n";

export function SearchBar() {
  const localQuery = useSignal(searchQuery.value);
  const filtersOpen = useSignal(false);

  const handleSubmit = (e: Event) => {
    e.preventDefault();
    searchQuery.value = localQuery.value;
    performSearch();
  };

  const tx = t.value;

  const CATEGORIES = [
    { value: "all", label: tx.categoryAll },
    { value: "anime", label: tx.categoryAnime },
    { value: "audio", label: tx.categoryAudio },
    { value: "literature", label: tx.categoryLiterature },
    { value: "live-action", label: tx.categoryLiveAction },
    { value: "pictures", label: tx.categoryPictures },
    { value: "software", label: tx.categorySoftware },
    { value: "games", label: tx.categoryGames },
  ];

  const SORTS = [
    { value: "date", label: tx.sortDate },
    { value: "seeders", label: tx.sortSeeders },
    { value: "leechers", label: tx.sortLeechers },
    { value: "downloads", label: tx.sortDownloads },
    { value: "size", label: tx.sortSize },
  ];

  const FILTERS = [
    { value: "no filter", label: tx.filterAll },
    { value: "trusted only", label: tx.filterTrusted },
    { value: "no remakes", label: tx.filterNoRemakes },
  ];

  return h("form", { class: "search-bar", onSubmit: handleSubmit },
    h("div", { class: "search-row" },
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
        type: "button",
        class: `btn btn-sm ${filtersOpen.value ? "btn-active" : ""}`,
        onClick: () => { filtersOpen.value = !filtersOpen.value; },
      },
        h(FilterIcon, { size: 14 }),
        tx.btnFilters,
        h(ChevronDownIcon, { size: 12, class: `filter-chevron${filtersOpen.value ? " open" : ""}` }),
      ),
      h("button", {
        type: "submit",
        class: "btn btn-primary",
        disabled: searchLoading.value,
      }, searchLoading.value ? tx.searching : tx.searchButton),
    ),
    h("div", { class: `filter-row${filtersOpen.value ? " open" : ""}` },
      h("select", {
        class: "select",
        value: searchCategory.value,
        onChange: (e: Event) => {
          searchCategory.value = (e.target as HTMLSelectElement).value;
        },
      }, CATEGORIES.map((c) =>
        h("option", { value: c.value }, c.label)
      )),
      h("select", {
        class: "select",
        value: searchFilter.value,
        onChange: (e: Event) => {
          searchFilter.value = (e.target as HTMLSelectElement).value;
        },
      }, FILTERS.map((f) =>
        h("option", { value: f.value }, f.label)
      )),
      h("select", {
        class: "select",
        value: searchSort.value,
        onChange: (e: Event) => {
          searchSort.value = (e.target as HTMLSelectElement).value;
        },
      }, SORTS.map((s) =>
        h("option", { value: s.value }, s.label)
      )),
      h("select", {
        class: "select",
        value: searchOrder.value,
        onChange: (e: Event) => {
          searchOrder.value = (e.target as HTMLSelectElement).value;
        },
      },
        h("option", { value: "desc" }, tx.orderDesc),
        h("option", { value: "asc" }, tx.orderAsc),
      ),
    ),
  );
}