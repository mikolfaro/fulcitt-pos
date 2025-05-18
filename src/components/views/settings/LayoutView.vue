<template>
  <div>
    <h2 class="text-xl font-semibold mb-4">Layout</h2>

    <div class="pb-4">
      <label class="label">
        <input
          type="checkbox"
          class="checkbox"
          :checked="layout.header.enabled"
          @click="layout.header.enabled = !layout.header.enabled"
        />
        <h3>Print header</h3>
      </label>
      <div
        v-if="layout.header.enabled"
        class="grid grid-cols-3"
      >
        <fieldset class="fieldset">
          <label class="label">Content</label>
          <input
            class="input"
            type="text"
            :value="layout.header.content"
            @change="(e: Event) => layout.header.content = (e?.currentTarget as HTMLInputElement | null)?.value as string"
          />
        </fieldset>

        <div></div>
        <div></div>

        <fieldset class="fieldset">
          <label for="header-font-size" class="label">
            Font size
          </label>
          <select
            id="header-font-size"
            class="select"
            :value="layout.header.font_size"
            @change="(e: Event) => layout.header.font_size = (e?.currentTarget as HTMLSelectElement | null)?.value as FontSize"
          >
            <option value="Small">Small</option>
            <option value="Normal">Normal</option>
            <option value="Large">Large</option>
          </select>
        </fieldset>

        <fieldset class="fieldset">
          <label for="header-justify" class="label">
            Justify
          </label>
          <select
            id="header-justify"
            class="select"
            :value="layout.header.justify"
            @change="(e: Event) => layout.header.justify = (e?.currentTarget as HTMLSelectElement | null)?.value as Justify"
          >
            <option value="Left">Left</option>
            <option value="Center">Center</option>
            <option value="Right">Right</option>
          </select>
        </fieldset>
      </div>
    </div>

    <div class="pb-4">
      <label class="label">
        <input
          type="checkbox"
          class="checkbox"
          :checked="layout.body.enabled"
          @click="layout.body.enabled = !layout.body.enabled"
        />
        <h3>Print body</h3>
      </label>
      <div
        v-if="layout.body.enabled"
        class="grid grid-cols-3"
      >
        <fieldset class="fieldset">
          <label for="body-font-size" class="label">
            Font size
          </label>
          <select
            id="body-font-size"
            class="select"
            :value="layout.body.font_size"
            @change="(e: Event) => layout.body.font_size = (e?.currentTarget as HTMLSelectElement | null)?.value as FontSize"
          >
            <option value="Small">Small</option>
            <option value="Normal">Normal</option>
            <option value="Large">Large</option>
          </select>
        </fieldset>

        <fieldset class="fieldset">
          <label for="body-justify" class="label">
            Justify
          </label>
          <select
            id="body-justify"
            class="select"
            :value="layout.body.justify"
            @change="(e: Event) => layout.body.justify = (e?.currentTarget as HTMLSelectElement | null)?.value as Justify"
          >
            <option value="Left">Left</option>
            <option value="Center">Center</option>
            <option value="Right">Right</option>
          </select>
        </fieldset>
      </div>
    </div>

    <div class="pb-4">
      <label class="label">
        <input
          type="checkbox"
          class="checkbox"
          :checked="layout.footer.enabled"
          @click="layout.footer.enabled = !layout.footer.enabled"
        />
        <h3>Print footer</h3>
      </label>
      <div
        v-if="layout.footer.enabled"
        class="grid grid-cols-3"
      >
        <fieldset class="fieldset">
          <label for="footer-font-size" class="label">
            Font size
          </label>
          <select
            id="footer-font-size"
            class="select"
            :value="layout.footer.font_size"
            @change="(e: Event) => layout.footer.font_size = (e?.currentTarget as HTMLSelectElement | null)?.value as FontSize"
          >
            <option value="Small">Small</option>
            <option value="Normal">Normal</option>
            <option value="Large">Large</option>
          </select>
        </fieldset>

        <fieldset class="fieldset">
          <label for="footer-justify" class="label">
            Justify
          </label>
          <select
            id="footer-justify"
            class="select"
            :value="layout.footer.justify"
            @change="(e: Event) => layout.footer.justify = (e?.currentTarget as HTMLSelectElement | null)?.value as Justify"
          >
            <option value="Left">Left</option>
            <option value="Center">Center</option>
            <option value="Right">Right</option>
          </select>
        </fieldset>
      </div>
    </div>

    <div>
      <button @click="saveLayout">
        Save
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref } from 'vue';
import { useMessagesStore } from '../../../stores/messagesStore';
import { AppMessage } from '../../../lib';

const messages = useMessagesStore();

type FontSize = 'Small' | 'Normal' | 'Large'
type Justify = 'Left' | 'Center' | 'Right'

interface SectionSettings {
  enabled: boolean
  // fontType: 'A' | 'B' | 'C'
  font_size: FontSize
  justify: Justify
  // bold: boolean
  // underline: 'None' | 'Single' | 'Double'
}

interface Layout {
  header: SectionSettings & { content: string },
  body: SectionSettings,
  footer: SectionSettings
}

const layout = ref<Layout>({
  header: {
    enabled: false,
    content: '',
    font_size: 'Normal',
    justify: 'Left'
  },
  body:{
    enabled: true,
    font_size: 'Normal',
    justify: 'Left'
  },
  footer: {
    enabled: false,
    font_size: 'Normal',
    justify: 'Left'
  },
})

const saveLayout = async function () {
  try {
    await invoke('save_print_layout', { layout: layout.value })
    messages.addSuccess("Layout saved")
  } catch (err) {
    messages.addMessage(err as AppMessage)
  }
}

onMounted(async function () {
  try {
    const loadedLayout = await invoke('get_print_layout')
    layout.value = loadedLayout as Layout
  } catch (err) {
    messages.addMessage(err as AppMessage)
  }
})
</script>
