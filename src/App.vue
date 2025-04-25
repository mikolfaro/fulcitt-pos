<template>
  <div class="flex flex-col h-screen">
    <Navbar />
    <main class="flex-grow overflow-auto">
      <RouterView />
    </main>
  </div>
</template>

<script lang="ts" setup>
  import Navbar from './components/Navbar.vue';
  import PosView from './components/PosView.vue';
</script>
