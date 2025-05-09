<template>
  <div class="navbar bg-base-300 shadow-md">
    <div class="flex-1">
      <a class="btn btn-ghost normal-case text-xl">POS App</a>
    </div>
    <div class="flex-none">
      <ul class="menu menu-horizontal px-1">
        <li><RouterLink to="/" class="btn btn-ghost">POS</RouterLink></li>
        <li><RouterLink to="/report" class="btn btn-ghost">Report</RouterLink></li>
        <li><RouterLink to="/settings" class="btn btn-ghost">Settings</RouterLink></li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router';
</script>
