<template>
  <div class="navbar bg-base-300 shadow-md">
    <div class="flex-1">
      <RouterLink
        to="/"
        class="btn btn-ghost normal-case text-xl"
      >{{ t('app.title') }}</RouterLink>
    </div>
    <div class="flex-none">
      <ul class="menu menu-horizontal px-1">
        <li><RouterLink to="/" class="btn btn-ghost">{{ t('pos.menu_title') }}</RouterLink></li>
        <li><RouterLink to="/report" class="btn btn-ghost">{{ t('reports.menu_title') }}</RouterLink></li>
        <li><RouterLink to="/settings" class="btn btn-ghost">{{ t('settings.menu_title') }}</RouterLink></li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>
