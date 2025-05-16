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
      <div v-if="layout.header.enabled">
        <fieldset class="fieldset">
          <label class="label">Content</label>
          <input
            class="input"
            type="text"
            :value="layout.header.content"
            @change="(e: Event) => layout.header.content = (e?.currentTarget as HTMLInputElement | null)?.value as string"
          />
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
      <div v-if="layout.body.enabled">
        Body more settings
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
      <div v-if="layout.footer.enabled">
        Footer more settings
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
import { ref } from 'vue';

interface SectionSettings {
  enabled: boolean
}

interface Layout {
  header: SectionSettings & { content: string },
  body: SectionSettings,
  footer: SectionSettings
}

const layout = ref<Layout>({
  header: {
    enabled: false,
    content: ''
  },
  body:{ enabled: true },
  footer: { enabled: false },
})

const saveLayout = function () {
  console.log(`New layout ${JSON.stringify(layout.value)}`)
}
</script>
