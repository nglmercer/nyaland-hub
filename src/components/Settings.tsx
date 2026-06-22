import { h } from "preact";
import { useSignal } from "@preact/signals";
import { settings, saveSettings } from "../stores/settings";
import { t, setLocale, locale } from "../i18n";
import type { Locale } from "../i18n/translations";
import { GlobeIcon } from "./icons";

export function Settings() {
  const localSettings = useSignal({ ...settings.value });
  const localLocale = useSignal(locale.value);
  const saved = useSignal(false);

  const tx = t.value;

  const handleSave = async () => {
    await saveSettings(localSettings.value);
    setLocale(localLocale.value);
    saved.value = true;
    setTimeout(() => { saved.value = false; }, 2000);
  };

  const update = (key: string, value: string | number | boolean) => {
    localSettings.value = { ...localSettings.value, [key]: value };
  };

  return h("div", { class: "settings" },
    h("h2", null, tx.settingsTitle),
    h("div", { class: "settings-form" },
      h("div", { class: "form-group" },
        h("label", null,
          h(GlobeIcon, { size: 14 }),
          tx.settingsLanguage,
        ),
        h("select", {
          class: "filter-select",
          value: localLocale.value,
          onChange: (e: Event) => {
            localLocale.value = (e.target as HTMLSelectElement).value as Locale;
          },
        },
          h("option", { value: "en" }, "English"),
          h("option", { value: "es" }, "Espanol"),
        ),
      ),
      h("div", { class: "form-group" },
        h("label", null, tx.settingsDownloadPath),
        h("input", {
          type: "text",
          value: localSettings.value.save_path,
          onInput: (e: Event) => update("save_path", (e.target as HTMLInputElement).value),
        }),
      ),
      h("div", { class: "form-group" },
        h("label", null, tx.settingsNyaaUrl),
        h("input", {
          type: "text",
          value: localSettings.value.nyaa_base_url,
          onInput: (e: Event) => update("nyaa_base_url", (e.target as HTMLInputElement).value),
        }),
      ),
      h("div", { class: "form-group" },
        h("label", null, tx.settingsMaxDown),
        h("input", {
          type: "number",
          value: String(localSettings.value.max_download_speed),
          onInput: (e: Event) => update("max_download_speed", Number((e.target as HTMLInputElement).value)),
        }),
      ),
      h("div", { class: "form-group" },
        h("label", null, tx.settingsMaxUp),
        h("input", {
          type: "number",
          value: String(localSettings.value.max_upload_speed),
          onInput: (e: Event) => update("max_upload_speed", Number((e.target as HTMLInputElement).value)),
        }),
      ),
      h("div", { class: "form-group" },
        h("label", null, tx.settingsMaxConn),
        h("input", {
          type: "number",
          value: String(localSettings.value.max_connections),
          onInput: (e: Event) => update("max_connections", Number((e.target as HTMLInputElement).value)),
        }),
      ),
      h("div", { class: "form-group" },
        h("label", null, tx.settingsMaxActive),
        h("input", {
          type: "number",
          value: String(localSettings.value.max_active_downloads),
          onInput: (e: Event) => update("max_active_downloads", Number((e.target as HTMLInputElement).value)),
        }),
      ),
      h("div", { class: "form-group" },
        h("label", { class: "checkbox-label" },
          h("input", {
            type: "checkbox",
            checked: localSettings.value.start_on_launch,
            onChange: (e: Event) => update("start_on_launch", (e.target as HTMLInputElement).checked),
          }),
          tx.settingsStartOnLaunch,
        ),
      ),
      h("div", { class: "form-actions" },
        h("button", {
          class: "btn btn-primary",
          onClick: handleSave,
        }, saved.value ? tx.settingsSaved : tx.btnSave),
      ),
    ),
  );
}
