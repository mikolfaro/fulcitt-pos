<template>
  <div class="p-4 md:p-6">
    <h1 class="text-2xl font-bold mb-4">
      {{ $t('settings-title') }}
    </h1>

    <div role="tablist" class="tabs tabs-lifted mb-[-1px]">
      <router-link
        v-for="r in subRoutes"
        role="tab"
        class="tab [--tab-border-color:oklch(var(--bc)/0.2)]"
        active-class="tab-active"
        :to="r.to"
      >
        {{ r.name }}
      </router-link>
      <a role="tab" class="tab [--tab-border-color:oklch(var(--bc)/0.2)] flex-grow !border-b"></a>
    </div>

    <div class="bg-base-100 border border-base-300 border-t-0 rounded-b-box p-6 min-h-[400px]">
      <router-view v-slot="{ Component, route }">
         <transition name="fade" mode="out-in">
            <keep-alive>
              <component :is="Component" :key="route.name" />
            </keep-alive>
         </transition>
      </router-view>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { useFluent } from 'fluent-vue';

const { $t } = useFluent()

const subRoutes = [
  { name: $t('settings-products-menu-title'), to: "/settings" },
  { name: $t('settings-layout-menu-title'), to: "/settings/layout" },
  { name: $t('settings-printer-menu-title'), to: "/settings/printer" },
]
</script>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.tab.tab-active {
  border-bottom-color: transparent !important;
  background-color: oklch(var(--b1)/var(--tw-bg-opacity,1));
}
</style>
