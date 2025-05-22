<template>
  <div class="flex flex-col h-screen">
    <NavBar />
    <main class="flex-grow overflow-auto">
      <RouterView />

      <MessageDisplay/>
    </main>
  </div>
</template>

<script lang="ts" setup>
import MessageDisplay from './components/MessageDisplay.vue';
import NavBar from './components/NavBar.vue';
</script>
