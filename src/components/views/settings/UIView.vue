<template>
  <div>
    <h2 class="text-xl font-semibold mb-4">
      {{ $t('settings-ui-title') }}
    </h2>

    <div class="card bg-base-200 shadow-md mt-8">
      <div class="card-body">
        <h3 class="card-title text-lg">
          {{ $t('settings-ui-theme-label') }}
        </h3>
        <p class="text-sm text-base-content/70 mb-4">
          Scegli il tema dell'applicazione
        </p>
        <div class="w-1/3 form-control">
          <select
            id="theme-selection"
            class="select select-bordered w-full"
            v-model="uiSettings.theme"
          >
            <option value="dark">{{ $t('settings-ui-theme-dark') }}</option>
            <option value="light">{{ $t('settings-ui-theme-light') }}</option>
            <option value="system">{{ $t('settings-ui-theme-system') }}</option>
          </select>
        </div>
      </div>
    </div>

    <div class="card bg-base-200 shadow-md mt-8">
      <div class="card-body">
        <h3 class="card-title text-lg">
          {{ $t('settings-ui-zoom-label') }}
        </h3>
        <p class="text-sm text-base-content/70 mb-4">
          Regola il livello di ingrandimento dell'interfaccia
        </p>
        <div class="w-1/3 form-control">
          <select
            id="zoom-selection"
            class="select select-bordered w-full"
            v-model="uiSettings.zoom"
          >
            <option value="90">{{ $t('settings-ui-zoom-small') }}</option>
            <option value="100">{{ $t('settings-ui-zoom-normal') }}</option>
            <option value="110">{{ $t('settings-ui-zoom-large') }}</option>
          </select>
        </div>
      </div>
    </div>

    <div class="pt-8">
      <button class="btn btn-success" @click="save">
        {{ $t('settings-ui-save-button') }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useFluent } from 'fluent-vue'
import { useMessagesStore } from '../../../stores/messagesStore'
import { useUiSettings } from '../../../stores/uiSettings'

const { $t } = useFluent()
const messages = useMessagesStore()
const { state: uiSettings, load: loadSettings, save: saveSettings } = useUiSettings()

async function save() {
  try {
    await saveSettings()
    messages.addSuccess($t('settings-ui-messages-settings-saved'))
  } catch (error) {
    console.error('Error saving UI settings:', error)
    messages.addUnknownError(error)
  }
}

onMounted(async () => {
  try {
    await loadSettings()
  } catch (error) {
    console.error('Error loading UI settings:', error)
  }
})
</script>
