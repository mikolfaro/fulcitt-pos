<template>
  <div class="flex flex-col h-screen">
    <NavBar />
    <main class="grow overflow-auto">
      <RouterView />

      <MessageDisplay />
    </main>
  </div>
</template>

<script lang="ts" setup>
import { onBeforeMount } from "vue";
import { useUiSettings } from "./stores/uiSettings";
import MessageDisplay from "./components/MessageDisplay.vue";
import NavBar from "./components/NavBar.vue";

const uiSettings = useUiSettings();

onBeforeMount(async () => {
  try {
    await uiSettings.load();
  } catch (error) {
    console.error("Error loading UI settings:", error);
  }
});
</script>
