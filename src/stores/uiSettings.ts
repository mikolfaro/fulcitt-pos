import { reactive, watch } from 'vue'
import { load as loadStore } from '@tauri-apps/plugin-store'

type Theme = 'dark' | 'light' | 'system'
type ZoomLevel = '100' | '150' | '200'

interface UISettings {
  theme: Theme
  zoom: ZoomLevel
}

const state = reactive<UISettings>({
  theme: 'system',
  zoom: '100'
})

let isInitialized = false

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
  const html = document.documentElement
  const zoomNum = parseInt(zoom)
  const scaleFactor = zoomNum / 100

  // Set base font-size for rem-based scaling
  html.style.fontSize = `${zoom}%`

  // Set CSS custom properties for fine-grained control
  html.style.setProperty('--zoom-scale', scaleFactor.toString())
  html.style.setProperty('--zoom-scale-inv', (1 / scaleFactor).toString())
}

const setupSystemThemeListener = () => {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  mediaQuery.addEventListener('change', () => {
    if (state.theme === 'system') {
      applyTheme('system')
    }
  })
}

const setupWatchers = () => {
  // Watch for changes and apply them immediately
  watch(() => state.theme, (newTheme) => {
    applyTheme(newTheme)
  })

  watch(() => state.zoom, (newZoom) => {
    applyZoom(newZoom)
  })
}

export const useUiSettings = function () {
  const load = async () => {
    try {
      const settingsStore = await loadStore('store.json', {
        autoSave: false,
        defaults: {
          'ui-theme': 'system',
          'ui-zoom': '100'
        }
      })

      const savedTheme = await settingsStore.get<Theme>('ui-theme')
      const savedZoom = await settingsStore.get<ZoomLevel>('ui-zoom')

      if (savedTheme) {
        state.theme = savedTheme
        applyTheme(savedTheme)
      }

      if (savedZoom) {
        state.zoom = savedZoom
        applyZoom(savedZoom)
      }

      // Setup watchers and listener only once
      if (!isInitialized) {
        setupWatchers()
        setupSystemThemeListener()
        isInitialized = true
      }
    } catch (error) {
      console.error('Error loading UI settings:', error)
      throw error
    }
  }

  const save = async () => {
    try {
      const settingsStore = await loadStore('store.json', {
        autoSave: false,
        defaults: {
          'ui-theme': 'system',
          'ui-zoom': '100',
        },
      })

      await settingsStore.set('ui-theme', state.theme)
      await settingsStore.set('ui-zoom', state.zoom)
      await settingsStore.save()
    } catch (error) {
      console.error('Error saving UI settings:', error)
      throw error
    }
  }

  return {
    state,
    load,
    save
  }
}
