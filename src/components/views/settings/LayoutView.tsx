import { invoke } from "@tauri-apps/api/core";
import { useFluent } from "fluent-vue";
import { defineComponent, onMounted, ref } from "vue";
import type { AppMessage } from "../../../lib";
import { useMessagesStore } from "../../../stores/messagesStore";
import { targetValue } from "../../../utils/inputs";

type FontSize = "Small" | "Normal" | "Large";
type Justify = "Left" | "Center" | "Right";

interface SectionSettings {
  enabled: boolean;
  // fontType: 'A' | 'B' | 'C'
  font_size: FontSize;
  justify: Justify;
  // bold: boolean
  // underline: 'None' | 'Single' | 'Double'
}

interface Layout {
  group_tickets_by_category: boolean;
  header: SectionSettings & { content: string };
  body: SectionSettings;
  footer: SectionSettings;
}

export default defineComponent({
  name: "LayoutView",
  setup() {
    const { $t } = useFluent();
    const messages = useMessagesStore();

    const layout = ref<Layout>({
      group_tickets_by_category: false,
      header: {
        enabled: false,
        content: "",
        font_size: "Normal",
        justify: "Left",
      },
      body: {
        enabled: true,
        font_size: "Normal",
        justify: "Left",
      },
      footer: {
        enabled: false,
        font_size: "Normal",
        justify: "Left",
      },
    });

    const saveLayout = async function () {
      try {
        await invoke("save_print_layout", { layout: layout.value });
        messages.addSuccess($t("Settings-layout-messages-layout-saved"));
      } catch (err) {
        messages.addMessage(err as AppMessage);
      }
    };

    onMounted(async function () {
      try {
        const loadedLayout = await invoke("get_print_layout");
        layout.value = loadedLayout as Layout;
      } catch (err) {
        messages.addMessage(err as AppMessage);
      }
    });

    const fontSizeSelect = (id: string, section: SectionSettings) => (
      <fieldset class="fieldset">
        <label for={id} class="label">
          {$t("settings-layout-font-size-label")}
        </label>
        <select
          id={id}
          class="select"
          value={section.font_size}
          onChange={(e) => {
            section.font_size = targetValue(e) as FontSize;
          }}
        >
          <option value="Small">{$t("settings-layout-font-size-small")}</option>
          <option value="Normal">
            {$t("settings-layout-font-size-normal")}
          </option>
          <option value="Large">{$t("settings-layout-font-size-large")}</option>
        </select>
      </fieldset>
    );

    const justifySelect = (id: string, section: SectionSettings) => (
      <fieldset class="fieldset">
        <label for={id} class="label">
          {$t("settings-layout-justify-label")}
        </label>
        <select
          id={id}
          class="select"
          value={section.justify}
          onChange={(e) => {
            section.justify = targetValue(e) as Justify;
          }}
        >
          <option value="Left">{$t("settings-layout-justify-left")}</option>
          <option value="Center">{$t("settings-layout-justify-center")}</option>
          <option value="Right">{$t("settings-layout-justify-right")}</option>
        </select>
      </fieldset>
    );

    const sectionToggle = (section: SectionSettings, label: string) => (
      <label class="label">
        <input
          type="checkbox"
          class="checkbox"
          checked={section.enabled}
          onClick={() => {
            section.enabled = !section.enabled;
          }}
        />
        <h3>{label}</h3>
      </label>
    );

    return () => (
      <div>
        <h2 class="text-xl font-semibold mb-4">
          {$t("settings-layout-title")}
        </h2>

        <div class="pb-4">
          <label class="label">
            <input
              type="checkbox"
              class="checkbox"
              checked={layout.value.group_tickets_by_category}
              onClick={() => {
                layout.value.group_tickets_by_category =
                  !layout.value.group_tickets_by_category;
              }}
            />
            <h3>{$t("settings-layout-print-group-tickets-by-category")}</h3>
          </label>
        </div>

        <div class="pb-4">
          {sectionToggle(
            layout.value.header,
            $t("settings-layout-print-header-label"),
          )}
          {layout.value.header.enabled && (
            <div class="grid grid-cols-3">
              <fieldset class="fieldset">
                <label class="label">
                  {$t("settings-layout-header-content-label")}
                </label>
                <input
                  class="input"
                  type="text"
                  value={layout.value.header.content}
                  onChange={(e) => {
                    layout.value.header.content = targetValue(e);
                  }}
                />
              </fieldset>

              <div></div>
              <div></div>

              {fontSizeSelect("header-font-size", layout.value.header)}
              {justifySelect("header-justify", layout.value.header)}
            </div>
          )}
        </div>

        <div class="pb-4">
          {sectionToggle(
            layout.value.body,
            $t("settings-layout-print-body-label"),
          )}
          {layout.value.body.enabled && (
            <div class="grid grid-cols-3">
              {fontSizeSelect("body-font-size", layout.value.body)}
              {justifySelect("body-justify", layout.value.body)}
            </div>
          )}
        </div>

        <div class="pb-4">
          {sectionToggle(
            layout.value.footer,
            $t("settings-layout-print-footer-label"),
          )}
          {layout.value.footer.enabled && (
            <div class="grid grid-cols-3">
              {fontSizeSelect("footer-font-size", layout.value.footer)}
              {justifySelect("footer-justify", layout.value.footer)}
            </div>
          )}
        </div>

        <div>
          <button class="btn btn-success" onClick={saveLayout}>
            {$t("settings-layout-save-button")}
          </button>
        </div>
      </div>
    );
  },
});
