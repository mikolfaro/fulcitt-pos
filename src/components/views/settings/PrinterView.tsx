import { invoke } from "@tauri-apps/api/core";
import { appDataDir } from "@tauri-apps/api/path";
import { openPath } from "@tauri-apps/plugin-opener";
import { load } from "@tauri-apps/plugin-store";
import { useFluent } from "fluent-vue";
import { defineComponent, onMounted, reactive, ref } from "vue";
import type { AppMessage } from "../../../lib";
import { useMessagesStore } from "../../../stores/messagesStore";
import { targetNumber, targetValue } from "../../../utils/inputs";

interface PrinterInfo {
  vendor_id: number;
  product_id: number;
  class: number;
  subclass: number;
  manufacturer: string;
  product: string;
}

type PrinterType = "None" | "File" | "Network" | "Usb";
type UsbPrinterId = {
  vendor_id: number;
  product_id: number;
};
type NetworkPrinterAddress = {
  ip: string;
  port: number;
};

const usbPrinterKey = (printer: UsbPrinterId) =>
  `${printer.vendor_id} ${printer.product_id}`;

export default defineComponent({
  name: "PrinterView",
  setup() {
    const { $t } = useFluent();
    const messages = useMessagesStore();

    const availableUsbPrinters = ref<PrinterInfo[]>([]);
    const printerOutputPath = ref<string>("");

    const printerTest = reactive<{
      text: string;
      type: PrinterType;
      id: UsbPrinterId | null;
      networkIp: string | null;
      networkPort: number | null;
    }>({
      text: $t("settings-printer-test-example", { date: new Date() }),
      type: "None",
      id: null,
      networkIp: null,
      networkPort: null,
    });

    const formatName = function (printer: PrinterInfo): string {
      const name = [printer.manufacturer, printer.product].join(" ");
      const vendorId = printer.vendor_id.toString(16).padStart(4, "0");
      const productId = printer.product_id.toString(16).padStart(4, "0");
      return `${name} (${vendorId}:${productId})`.trim();
    };

    async function triggerPrint() {
      if (printerTest.type === "None") {
        messages.addInvalidInput(
          $t("settings-printer-messages-no-printer-selected"),
        );
        return;
      } else if (printerTest.type === "Usb" && !printerTest.id) {
        messages.addInvalidInput(
          $t("settings-printer-messages-usb-device-not-selected"),
        );
        return;
      }

      if (!printerTest.text) {
        messages.addInvalidInput(
          $t("settings-printer-messages-no-text-to-print"),
        );
        return;
      }

      try {
        const printerAddress =
          printerTest.networkIp && printerTest.networkPort
            ? { ip: printerTest.networkIp, port: printerTest.networkPort }
            : null;

        await invoke("test_printer", {
          printerType: printerTest.type,
          printerId: printerTest.id,
          printerAddress,
          textToPrint: printerTest.text,
        });
        messages.addSuccess($t("settings-printer-messages-print-success"));

        printerTest.text = $t("settings-printer-test-example", {
          date: new Date(),
        });
      } catch (error) {
        if (typeof error === "string") {
          messages.addUnknownError(error);
        } else {
          messages.addMessage(error as AppMessage);
        }
      }
    }

    async function triggerOpenCashDrawer() {
      try {
        await invoke("open_cash_drawer");
      } catch (error) {
        if (typeof error === "string") {
          messages.addUnknownError(error);
        } else {
          messages.addMessage(error as AppMessage);
        }
      }

      messages.addMessage({
        type: "success",
        message: "settings-printer-messages-opening-app-folder",
      });
    }

    async function save() {
      try {
        if (printerTest.type === "None") {
          messages.addInvalidInput(
            $t("settings-printer-messages-no-printer-selected"),
          );
          return;
        } else if (printerTest.type === "Usb" && !printerTest.id) {
          messages.addInvalidInput(
            $t("settings-printer-messages-usb-device-not-selected"),
          );
          return;
        }

        const printerAddress =
          printerTest.networkIp && printerTest.networkPort
            ? { ip: printerTest.networkIp, port: printerTest.networkPort }
            : null;

        await invoke("save_printer_selection", {
          printerType: printerTest.type,
          printerId: printerTest.id,
          printerAddress,
        });

        messages.addSuccess(
          $t("settings-printer-messages-printer-device-saved"),
        );
      } catch (error) {
        console.error(error);

        if (typeof error === "string") {
          messages.addUnknownError(error);
        } else {
          messages.addMessage(error as AppMessage);
        }
      }
    }

    async function openPrinterFile() {
      try {
        await openPath(printerOutputPath.value);
        messages.addSuccess(
          $t("settings-printer-messages-opening-printer-file"),
        );
      } catch (err) {
        messages.addUnknownError(err);
      }
    }

    async function openPrinterFolder() {
      try {
        const dataDir = await appDataDir();
        await openPath(dataDir);
        messages.addSuccess($t("settings-printer-messages-opening-app-folder"));
      } catch (err) {
        messages.addUnknownError(err);
      }
    }

    async function refreshAvailablePrinters() {
      try {
        const usbPrinters = await invoke<PrinterInfo[]>("list_usb_devices");

        availableUsbPrinters.value = usbPrinters.sort(function (a, b) {
          const nameA = formatName(a);
          const nameB = formatName(b);

          if (nameA < nameB) {
            return -1;
          } else if (nameA > nameB) {
            return 1;
          } else {
            return 0;
          }
        });
      } catch (err) {
        messages.addUnknownError(err);
      }
    }

    onMounted(async () => {
      // Set up printer output path
      const dataDir = await appDataDir();
      printerOutputPath.value = `${dataDir}/printer_output.txt`;

      const settingsStore = await load("store.json", {
        autoSave: false,
        defaults: {
          printerType: "None",
          printerId: null,
        },
      });
      const printerType = await settingsStore.get<PrinterType>("printer-type");
      const printerId = await settingsStore.get<UsbPrinterId>("printer-id");
      const printerAddress =
        await settingsStore.get<NetworkPrinterAddress>("printer-address");

      await refreshAvailablePrinters();

      if (printerType === "Usb" && printerId) {
        printerTest.id =
          availableUsbPrinters.value.find((p) => {
            return (
              p.product_id == printerId.product_id &&
              p.vendor_id == printerId.vendor_id
            );
          }) || null;

        printerTest.type = printerType;
      } else if (printerType === "Network" && printerAddress) {
        printerTest.type = "Network";
        printerTest.networkIp = printerAddress.ip;
        printerTest.networkPort = printerAddress.port;
      } else if (printerType === "File") {
        printerTest.type = "File";
      } else {
        printerTest.type = "None";
      }
    });

    return () => (
      <div>
        <h2 class="text-xl font-semibold mb-4">
          {$t("settings-printer-title")}
        </h2>
        <div class="card bg-base-200 shadow-md mt-8">
          <div class="card-body">
            <h3 class="card-title text-lg">
              {$t("settings-printer-printer-selection-title")}
            </h3>
            <p class="text-sm text-base-content/70 mb-4">
              {$t("settings-printer-printer-selection-text")}
            </p>
            <div class="flex flex-col justify-between">
              <div class="w-1/3 form-control mb-2">
                <label class="label">
                  {$t("settings-printer-type-selection-label")}
                </label>
                <select
                  class="select w-full"
                  value={printerTest.type}
                  onChange={(e) => {
                    printerTest.type = targetValue(e) as PrinterType;
                  }}
                >
                  <option value="None">
                    {$t("settings-printer-no-printer-selected")}
                  </option>
                  <option value="File">Stampante virtuale</option>
                  <option value="Network">Stampante di rete</option>
                  <option value="Usb">Stampante USB</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        {printerTest.type === "File" && (
          <div class="card bg-base-200 shadow-md mt-8">
            <div class="card-body">
              <h3 class="card-title text-lg">
                {$t("settings-printer-virtual-printer-output-title")}
              </h3>

              <div class="flex flex-col gap-2">
                <p class="text-sm text-base-content/70">
                  {$t("settings-printer-virtual-printer-description")}
                </p>
                <code class="text-xs bg-base-300 p-2 rounded">
                  {printerOutputPath.value}
                </code>

                <div class="flex gap-2 mt-2">
                  <button
                    class="btn btn-sm btn-primary btn-outline"
                    onClick={openPrinterFile}
                  >
                    {$t("settings-printer-virtual-printer-open-file-button")}
                  </button>
                  <button
                    class="btn btn-sm btn-primary btn-outline"
                    onClick={openPrinterFolder}
                  >
                    {$t(
                      "settings-printer-virtual-printer-open-directory-button",
                    )}
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

        {printerTest.type === "Usb" && (
          <div class="card bg-base-200 shadow-md mt-8">
            <div class="card-body">
              <h3 class="card-title text-lg">
                {$t("settings-printer-usb-device-selection")}
              </h3>

              <div class="flex flex-col justify-between">
                <div class="w-1/3 form-control mb-2">
                  <label class="label">
                    {$t("settings-printer-usb-device-selection-label")}
                  </label>
                  <select
                    class="select w-full"
                    value={printerTest.id ? usbPrinterKey(printerTest.id) : ""}
                    onChange={(e) => {
                      const key = targetValue(e);
                      printerTest.id =
                        availableUsbPrinters.value.find(
                          (p) => usbPrinterKey(p) === key,
                        ) ?? null;
                    }}
                  >
                    {availableUsbPrinters.value.map((option) => (
                      <option
                        key={usbPrinterKey(option)}
                        value={usbPrinterKey(option)}
                      >
                        {formatName(option)}
                      </option>
                    ))}
                  </select>
                </div>
              </div>
            </div>
          </div>
        )}

        {printerTest.type === "Network" && (
          <div class="card bg-base-200 shadow-md mt-8">
            <div class="card-body">
              <h3 class="card-title text-lg">
                {$t("settings-printer-network-printer-title")}
              </h3>

              <div class="flex flex-col justify-between">
                <div class="w-1/3 form-control mb-2">
                  <label class="label">
                    {$t("settings-printer-network-printer-address-label")}
                  </label>
                  <input
                    type="text"
                    class="input input-bordered w-full"
                    value={printerTest.networkIp ?? ""}
                    onInput={(e) => {
                      printerTest.networkIp = targetValue(e);
                    }}
                  />
                </div>
                <div class="w-1/3 form-control mb-2">
                  <label class="label">
                    {$t("settings-printer-network-printer-port-label")}
                  </label>
                  <input
                    type="number"
                    step="1"
                    class="input input-bordered w-full"
                    value={printerTest.networkPort ?? ""}
                    onInput={(e) => {
                      printerTest.networkPort = targetNumber(e);
                    }}
                  />
                </div>
              </div>
            </div>
          </div>
        )}

        {printerTest.type !== "None" && (
          <div class="card bg-base-200 shadow-md mt-8">
            <div class="card-body">
              <h3 class="card-title text-lg">
                {$t("settings-printer-test-printer-title")}
              </h3>

              <div class="flex flex-col justify-between">
                <div class="w-1/3 form-control mb-2">
                  <label class="label">
                    {$t("settings-printer-text-to-be-printed-label")}
                  </label>
                  <input
                    id="textInput"
                    type="text"
                    class="input input-bordered w-full"
                    value={printerTest.text}
                    onInput={(e) => {
                      printerTest.text = targetValue(e);
                    }}
                  />
                </div>
              </div>

              <div class="card-actions">
                <button class="btn btn-primary" onClick={triggerPrint}>
                  {$t("settings-printer-test-print-button")}
                </button>
                <button
                  class="btn btn-secondary"
                  onClick={triggerOpenCashDrawer}
                >
                  {$t("settings-printer-open-cash-drawer-button")}
                </button>
              </div>
            </div>
          </div>
        )}

        <div class="pt-8">
          <button class="btn btn-success" onClick={save}>
            {$t("settings-printer-save-button")}
          </button>
        </div>
      </div>
    );
  },
});
