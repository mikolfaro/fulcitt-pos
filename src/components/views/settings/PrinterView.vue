<template>
  <div>
    <h2 class="text-xl font-semibold mb-4">
      {{ $t('settings-printer-title') }}
    </h2>
    <div class="card bg-base-200 shadow-md mt-8">
      <div class="card-body">
        <h3 class="card-title text-lg">
          {{ $t('settings-printer-printer-selection-title') }}
        </h3>
        <p class="text-sm text-base-content/70 mb-4">
          {{ $t('settings-printer-printer-selection-text') }}
        </p>
        <div class="flex flex-col justify-between">
          <div class="w-1/3 form-control mb-2">
            <label class="label">
              {{ $t('settings-printer-type-selection-label') }}
            </label>
            <select class="select w-full" v-model="printerTest.type">
              <option value="NONE">Nessuna stampate selezionata</option>
              <option value="FILE">Stampante virtuale</option>
              <option value="USB">Stampante USB</option>
            </select>
          </div>

          <p class="text-sm text-base-content/70">
            {{ $t('settings-printer-virtual-printer-description') }}
          </p>
        </div>
      </div>
    </div>

    <div class="card bg-base-200 shadow-md mt-8" v-if="printerTest.type === 'USB'">
      <div class="card-body">
        <h3 class="card-title text-lg">
          {{ $t('settings.printer-usb-device-selection') }}
        </h3>

        <div class="flex flex-col justify-between">
          <div class="w-1/3 form-control mb-2">
            <label class="label">
              {{ $t('settings-printer-usb-device-selection-label') }}
            </label>
            <select class="select w-full">
              <option v-for="option in availableUsbPrinters" :key="`${option.vendor_id} ${option.product_id}`">
                {{ formatName(option) }}
              </option>
            </select>
          </div>
        </div>
      </div>
    </div>

    <div class="card bg-base-200 shadow-md mt-8" v-id="printerTest.type !== 'NONE'">
      <div class="card-body">
        <h3 class="card-title text-lg">
          {{ $t('settings-printer-test-printer-title') }}
        </h3>

        <div class="flex flex-col justify-between">
          <div class="w-1/3 form-control mb-2">
            <label class="label">
              {{ $t('settings-printer-text-to-be-printed-label') }}
            </label>
            <input id="textInput" type="text" class="input input-bordered w-full" v-model="printerTest.text" />
          </div>
        </div>

        <div class="card-actions">
          <button class="btn btn-primary" @click="triggerPrint">
            {{ $t('settings-printer-test-print-button') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { load } from '@tauri-apps/plugin-store'
import { onMounted, reactive, ref } from 'vue'
import { useFluent } from 'fluent-vue'
import { useMessagesStore } from '../../../stores/messagesStore'
import { AppMessage } from '../../../lib'

interface PrinterInfo {
  vendor_id: number,
  product_id: number,
  class: number,
  subclass: number,
  manufacturer: string,
  product: string,
}

const { $t } = useFluent()
const messages = useMessagesStore()

const availableUsbPrinters = ref<PrinterInfo[]>([])

type PrinterType = "NONE" | "FILE" | "USB"
type UsbPrinterId = {
  vendor_id: number,
  product_id: number,
}

const printerTest = reactive<{
  text: string,
  printerType: PrinterType,
  id: UsbPrinterId | null
}>({
  text: $t('settings-printer-test-example', { date: new Date() }),
  printerType: "NONE",
  id: null,
})

const formatName = function (printer: PrinterInfo): string {
  const name = [printer.manufacturer, printer.product].join(' ')
  const vendorId = printer.vendor_id.toString(16).padStart(4, "0")
  const productId = printer.product_id.toString(16).padStart(4, "0")
  return `${name} (${vendorId}:${productId})`.trim()
}

async function triggerPrint() {
  if (printerTest.printerType === 'NONE') {
    messages.addInvalidInput($t('settings-printer-messages-no-printer-selected'));
    return;
  } else if (printerTest.printerType === 'USB' && !printerTest.id) {
    messages.addInvalidInput($t('settings-printer-messages-usb-device-not-selected'));
    return;
  }

  // if (!printerTest.device || !printerTest.text) {
  //   messages.addInvalidInput($t('settings-printer-messages-device-cannot-be-empty'));
  //   return;
  // }
  //
  // try {
  //   await invoke('test_print_raw_file', {
  //       device: printerTest.device,
  //       textToPrint: printerTest.text
  //   })
  //   messages.addSuccess($t('settings-printer-messages-print-success'))
  //
  //   printerTest.text = $t('settings-printer-test-example', { date: new Date() })
  // } catch (error) {
  //   if (typeof error === 'string') {
  //     messages.addUnknownError(error)
  //   } else {
  //     messages.addMessage(error as AppMessage);
  //   }
  // }
}

async function save() {
  // try {
  //   await invoke('save_printer_device', { device: printerTest.device })
  //   messages.addSuccess($t('settings-printer-messages-printer-device-saved'))
  // } catch (error) {
  //   console.error(error)
  //
  //   if (typeof error === 'string') {
  //     messages.addUnknownError(error)
  //   } else {
  //     messages.addMessage(error as AppMessage);
  //   }
  // }
}

async function refreshAvailablePrinters() {
  try {
    const usbPrinters = await invoke<PrinterInfo[]>('list_usb_devices')

    const deviceClasses = usbPrinters.map((device) => {
      return {
        ...device,
        class: device.class.toString(16),
        subClass: device.subclass.toString(16),
      }
    })

    console.log("Available devices found", deviceClasses)

    availableUsbPrinters.value = usbPrinters.sort(function (a, b) {
      const nameA = formatName(a)
      const nameB = formatName(b)

      if (nameA < nameB) {
        return -1
      } else if (nameA > nameB) {
        return 1
      } else {
        return 0
      }
    })
  } catch (err) {
    messages.addUnknownError(err)
  }
}

onMounted(async () => {
  const settingsStore = await load('store.json', { autoSave: false });
  const device = await settingsStore.get('printer-device')

  console.log("Restored device", device)

  refreshAvailablePrinters()

  // if (typeof device === 'object') {
  //   printerTest.device = device as Device
  // }
  //
  // await refreshAvailableDevices()
})
</script>
