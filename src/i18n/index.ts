import { signal, computed } from "@preact/signals";
import { translations, type Locale, type TranslationKeys } from "./translations";

export const locale = signal<Locale>("en");

export const t = computed(() => translations[locale.value]);

export function setLocale(l: Locale) {
  locale.value = l;
}
