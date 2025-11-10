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

      <div v-if="printerTest.type === 'FILE'">
        <p>
          Una stampante virtuale non è una vera e propria "stampante".
          <br />
          I ticket sono salvati in un file TXT.
        </p>
      </div>

      <div v-if="printerTest.type === 'USB'">
        <h3 class="card-title text-lg">
          {{ $t('settings.printer-usb-device-selection') }}
        </h3>

        <div class="flex flex-col justify-between">
          <div class="w-1/3 form-control mb-2">
            <label class="label">
              {{ $t('settings-printer-usb-device-selection-label') }}
            </label>
            <select class="select w-full" v-model="printerTest.type">
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

// interface Device {
//   vendor_id: number,
//   product_id: number,
//   vendor_name: string,
//   product_name: string,
// }

const { $t } = useFluent()
const messages = useMessagesStore()

// const availableDevices = ref<Device[]>([])

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

// const formatName = function (device: Device): string {
//   const name = `${device.vendor_name} ${device.product_name}`
//   const vendorId = device.vendor_id.toString(16).padStart(4, "0")
//   const productId = device.product_id.toString(16).padStart(4, "0")
//   return `${name} (${vendorId}:${productId})`.trim()
// }

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

async function refreshAvailableDevices() {
  try {
    const devices = await invoke('list_usb_devices')

    const deviceClasses = devices.map((device) => {
      return {
        classCode: device.class_code.toString(16),
        subClassCode: device.sub_class_code.toString(16),
      }
    })

    console.log("Available devices found", deviceClasses)

    // availableDevices.value = (devices as Device[]).sort(function (a, b) {
    //   const nameA = formatName(a)
    //   const nameB = formatName(b)
    //
    //   if (nameA < nameB) {
    //     return -1
    //   } else if (nameA > nameB) {
    //     return 1
    //   } else {
    //     return 0
    //   }
    // })
  } catch (err) {
    messages.addUnknownError(err)
  }
}

onMounted(async () => {
  const settingsStore = await load('store.json', { autoSave: false });
  const device = await settingsStore.get('printer-device')

  console.log("REstored device", device)

  refreshAvailableDevices()

  // if (typeof device === 'object') {
  //   printerTest.device = device as Device
  // }
  //
  // await refreshAvailableDevices()
})
</script>
