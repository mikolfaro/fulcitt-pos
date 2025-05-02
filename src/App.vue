<template>
  <div class="flex flex-col h-screen">
    <NavBar />
    <main class="flex-grow overflow-auto">
      <RouterView />
    </main>
  </div>
</template>

<script lang="ts" setup>
  import NavBar from './components/NavBar.vue';
</script>
