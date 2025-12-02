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
import { load } from '@tauri-apps/plugin-store'
import { onMounted, reactive, watch } from 'vue'
import { useFluent } from 'fluent-vue'
import { useMessagesStore } from '../../../stores/messagesStore'

const { $t } = useFluent()
const messages = useMessagesStore()

type Theme = 'dark' | 'light' | 'system'
type ZoomLevel = '90' | '100' | '110'

interface UISettings {
  theme: Theme
  zoom: ZoomLevel
}

const uiSettings = reactive<UISettings>({
  theme: 'system',
  zoom: '100'
})

const applyTheme = (theme: Theme) => {
  const html = document.documentElement

  if (theme === 'system') {
    // Check system preference
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    html.setAttribute('data-theme', prefersDark ? 'dark' : 'light')
  } else {
    html.setAttribute('data-theme', theme)
  }
}

const applyZoom = (zoom: ZoomLevel) => {
  const body = document.body
  body.style.zoom = `${zoom}%`
}

// Watch for changes and apply them immediately
watch(() => uiSettings.theme, (newTheme) => {
  applyTheme(newTheme)
})

watch(() => uiSettings.zoom, (newZoom) => {
  applyZoom(newZoom)
})

// Listen for system theme changes
const setupSystemThemeListener = () => {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  mediaQuery.addEventListener('change', () => {
    if (uiSettings.theme === 'system') {
      applyTheme('system')
    }
  })
}

async function save() {
  try {
    const settingsStore = await load('store.json', { autoSave: false })

    await settingsStore.set('ui-theme', uiSettings.theme)
    await settingsStore.set('ui-zoom', uiSettings.zoom)
    await settingsStore.save()

    messages.addSuccess($t('settings-ui-messages-settings-saved'))
  } catch (error) {
    console.error('Error saving UI settings:', error)
    messages.addUnknownError(error)
  }
}

onMounted(async () => {
  try {
    const settingsStore = await load('store.json', {
      autoSave: false,
      defaults: {
        'ui-theme': 'system',
        'ui-zoom': '100'
      }
    })

    const savedTheme = await settingsStore.get<Theme>('ui-theme')
    const savedZoom = await settingsStore.get<ZoomLevel>('ui-zoom')

    if (savedTheme) {
      uiSettings.theme = savedTheme
      applyTheme(savedTheme)
    }

    if (savedZoom) {
      uiSettings.zoom = savedZoom
      applyZoom(savedZoom)
    }

    // Setup listener for system theme changes
    setupSystemThemeListener()
  } catch (error) {
    console.error('Error loading UI settings:', error)
  }
})
</script>
