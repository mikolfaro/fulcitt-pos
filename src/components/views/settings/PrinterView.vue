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
              <option value="None">Nessuna stampate selezionata</option>
              <option value="File">Stampante virtuale</option>
              <option value="Usb">Stampante USB</option>
            </select>
          </div>

          <p class="text-sm text-base-content/70">
            {{ $t('settings-printer-virtual-printer-description') }}
          </p>
        </div>
      </div>
    </div>

    <div class="card bg-base-200 shadow-md mt-8" v-if="printerTest.type === 'File'">
      <div class="card-body">
        <h3 class="card-title text-lg">
          Virtual Printer Output
        </h3>

        <div class="flex flex-col gap-2">
          <p class="text-sm text-base-content/70">
            The virtual printer writes to a file in your app data directory:
          </p>
          <code class="text-xs bg-base-300 p-2 rounded">{{ printerOutputPath }}</code>

          <div class="flex gap-2 mt-2">
            <button class="btn btn-sm btn-outline" @click="openPrinterFile">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              Open File
            </button>
            <button class="btn btn-sm btn-outline" @click="openPrinterFolder">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
              </svg>
              Open Folder
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="card bg-base-200 shadow-md mt-8" v-if="printerTest.type === 'Usb'">
      <div class="card-body">
        <h3 class="card-title text-lg">
          {{ $t('settings.printer-usb-device-selection') }}
        </h3>

        <div class="flex flex-col justify-between">
          <div class="w-1/3 form-control mb-2">
            <label class="label">
              {{ $t('settings-printer-usb-device-selection-label') }}
            </label>
            <select class="select w-full" v-model="printerTest.id">
              <option v-for="option in availableUsbPrinters" :key="`${option.vendor_id} ${option.product_id}`"
                :value="option">
                {{ formatName(option) }}
              </option>
            </select>
          </div>
        </div>
      </div>
    </div>

    <div class="card bg-base-200 shadow-md mt-8" v-id="printerTest.type !== 'None'">
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

    <div>
      <button class="btn btn-positive" @click="save">
        {{ $t('settings-printer-save-button') }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { appDataDir } from '@tauri-apps/api/path'
import { openPath } from '@tauri-apps/plugin-opener'
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
const printerOutputPath = ref<string>('')

type PrinterType = "None" | "File" | "Usb"
type UsbPrinterId = {
  vendor_id: number,
  product_id: number,
}

const printerTest = reactive<{
  text: string,
  type: PrinterType,
  id: UsbPrinterId | null
}>({
  text: $t('settings-printer-test-example', { date: new Date() }),
  type: "None",
  id: null,
})

const formatName = function (printer: PrinterInfo): string {
  const name = [printer.manufacturer, printer.product].join(' ')
  const vendorId = printer.vendor_id.toString(16).padStart(4, "0")
  const productId = printer.product_id.toString(16).padStart(4, "0")
  return `${name} (${vendorId}:${productId})`.trim()
}

async function triggerPrint() {
  if (printerTest.type === 'None') {
    messages.addInvalidInput($t('settings-printer-messages-no-printer-selected'));
    return;
  } else if (printerTest.type === 'Usb' && !printerTest.id) {
    messages.addInvalidInput($t('settings-printer-messages-usb-device-not-selected'));
    return;
  }

  if (!printerTest.text) {
    messages.addInvalidInput($t('settings-printer-messages-no-text-to-print'));
    return;
  }

  try {
    await invoke('test_printer', {
      printerType: printerTest.type,
      printerId: printerTest.id,
      textToPrint: printerTest.text
    })
    messages.addSuccess($t('settings-printer-messages-print-success'))

    printerTest.text = $t('settings-printer-test-example', { date: new Date() })
  } catch (error) {
    if (typeof error === 'string') {
      messages.addUnknownError(error)
    } else {
      messages.addMessage(error as AppMessage);
    }
  }
}

async function save() {
  try {
    if (printerTest.type === 'None') {
      messages.addInvalidInput($t('settings-printer-messages-no-printer-selected'));
      return;
    } else if (printerTest.type === 'Usb' && !printerTest.id) {
      messages.addInvalidInput($t('settings-printer-messages-usb-device-not-selected'));
      return;
    }

    await invoke('save_printer_selection', {
      printerType: printerTest.type,
      printerId: printerTest.id,
    })

    messages.addSuccess($t('settings-printer-messages-printer-device-saved'))
  } catch (error) {
    console.error(error)

    if (typeof error === 'string') {
      messages.addUnknownError(error)
    } else {
      messages.addMessage(error as AppMessage);
    }
  }
}

async function openPrinterFile() {
  try {
    await openPath(printerOutputPath.value)
    messages.addSuccess('Opening printer output file...')
  } catch (err) {
    messages.addUnknownError(err)
  }
}

async function openPrinterFolder() {
  try {
    const dataDir = await appDataDir()
    await openPath(dataDir)
    messages.addSuccess('Opening app data folder...')
  } catch (err) {
    messages.addUnknownError(err)
  }
}

async function refreshAvailablePrinters() {
  try {
    const usbPrinters = await invoke<PrinterInfo[]>('list_usb_devices')

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
  // Set up printer output path
  const dataDir = await appDataDir()
  printerOutputPath.value = `${dataDir}/printer_output.txt`

  const settingsStore = await load('store.json', { autoSave: false });
  const printerType = await settingsStore.get<PrinterType>('printer-type') || 'None'
  const printerId = await settingsStore.get<UsbPrinterId>('printer-id')

  await refreshAvailablePrinters()

  if (printerType === 'Usb' && printerId) {
    printerTest.id = availableUsbPrinters.value.find((p) => {
      return p.product_id == printerId.product_id && p.vendor_id == printerId.vendor_id
    }) || null

    printerTest.type = printerType
  } else if (printerType === 'File') {
    printerTest.type = 'File'
  } else {
    printerTest.type = 'None'
  }
})
</script>
