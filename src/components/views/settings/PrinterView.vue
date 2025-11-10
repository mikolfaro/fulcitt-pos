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
        </div>
      </div>
    </div>

    <div class="card bg-base-200 shadow-md mt-8" v-if="printerTest.type === 'FILE'">
      <div class="card-body">
        <p class="text-sm text-base-content/70">
          {{ $t('settings-printer-virtual-printer-description') }}
        </p>
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
              <option v-for="option in availableUsbPrinters">
                {{ formatName(option) }}
              </option>
            </select>
          </div>
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

interface Printer {
  vendor_id: number,
  product_id: number,
  class: number,
  subclass: number,
  manufacturer: string,
  product: string,
}

const { $t } = useFluent()
const messages = useMessagesStore()

const availableUsbPrinters = ref<Printer[]>([])

type PrinterType = "NONE" | "FILE" | "USB"

const printerTest = reactive<{
  text: string,
  type: PrinterType
}>({
  text: "",
  type: "NONE"
})

// const printerTest = reactive<{
//   text: string,
//   // device: Device | null
// }>({
//   // device: null,
//   text: $t('settings-printer-test-example', { date: new Date() }),
// })

const formatName = function (printer: Printer): string {
  const name = [printer.manufacturer, printer.product].join(' ')
  const vendorId = printer.vendor_id.toString(16).padStart(4, "0")
  const productId = printer.product_id.toString(16).padStart(4, "0")
  return `${name} (${vendorId}:${productId})`.trim()
}

async function triggerPrint() {
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
    const usbPrinters = await invoke<Printer[]>('list_usb_devices')

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
