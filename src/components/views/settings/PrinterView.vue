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
              {{ $t('settings-printer-device-path-label') }}
            </label>
            <input
              id="devicePathInput"
              type="text"
              class="input input-bordered w-full font-mono"
              v-model="printerTest.devicePath"
            />
          </div>
          <div class="w-1/3 form-control mb-4">
            <label class="label">
              {{ $t('settings-printer-text-to-be-printed_label') }}
            </label>
            <input
              id="textInput"
              type="text"
              class="input input-bordered w-full"
              v-model="printerTest.text"
            />
          </div>
          <div class="flex gap-4">
            <button class="btn btn-primary" @click="triggerPrint">
              {{ $t('settings-printer-test-print-button') }}
            </button>
            <button class="btn btn-success" @click="save">
              {{ $t('settings-printer-save-button') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { load } from '@tauri-apps/plugin-store'
import { onMounted, reactive } from 'vue'
import { useFluent } from 'fluent-vue'
import { useMessagesStore } from '../../../stores/messagesStore'
import { AppMessage } from '../../../lib'

const { $t } = useFluent()
const messages = useMessagesStore()

const printerTest = reactive({
  devicePath: '/dev/usb/lp0',
  text: $t('settings-printer-test-example', { date: new Date() }),
})

async function triggerPrint() {
  if (!printerTest.devicePath || !printerTest.text) {
    messages.addInvalidInput($t('settings-printer-messages-device-cannot-be-empty'));
    return;
  }

  try {
    await invoke('test_print_raw_file', {
        devicePath: printerTest.devicePath,
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
    await invoke('save_printer_device', { devicePath: printerTest.devicePath })
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

onMounted(async () => {
  const settingsStore = await load('store.json', { autoSave: false });
  const devicePath = await settingsStore.get('printer-device')
  if (typeof devicePath === 'string') {
    printerTest.devicePath = devicePath
  }
})
</script>
