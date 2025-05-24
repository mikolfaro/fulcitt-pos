<template>
  <div class="navbar bg-base-300 shadow-md">
    <div class="flex-1">
      <RouterLink
        to="/"
        class="btn btn-ghost normal-case text-xl"
      >{{ $t('app-title') }}</RouterLink>
    </div>
    <div class="flex-none">
      <ul class="menu menu-horizontal px-1">
        <li><RouterLink to="/" class="btn btn-ghost">{{ $t('pos-menu-title') }}</RouterLink></li>
        <li><RouterLink to="/report" class="btn btn-ghost">{{ $t('reports-menu-title') }}</RouterLink></li>
        <li><RouterLink to="/settings" class="btn btn-ghost">{{ $t('settings-menu-title') }}</RouterLink></li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { useFluent } from 'fluent-vue'

const { $t } = useFluent()
</script>
