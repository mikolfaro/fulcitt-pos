<template>
  <div class="grid grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
    <button v-for="product in products" :aria-disabled="allowAddToCart" :key="product.id"
      class="card bg-base-300 shadow-md hover:shadow-lg hover:bg-base-200 transition-shadow duration-200 ease-in-out cursor-pointer"
      :class="{ 'text-secondary': refundModeActive }" @click="() => { allowAddToCart && addToCart(product) }">
      <div class="flex card-body justify-between items-center p-3">
        <h3 class="card-title text-sm leading-tight">{{ product.name }}</h3>
        <div class="flex items-center gap-1 mt-1">
          <span class="text-md font-semibold">{{ formatCurrency(product.price) }}</span>
          <kbd v-if="shortcuts.keyForProduct(product.id)" class="kbd kbd-xs opacity-60">
            {{ shortcuts.keyForProduct(product.id) }}
          </kbd>
        </div>
      </div>
    </button>
  </div>
</template>

<script setup lang="ts">
import { useProductShortcutsStore } from "../stores/productShortcuts";
import { Product } from "../lib";

defineProps<{
  products: Product[];
  allowAddToCart: boolean;
  refundModeActive: boolean;
}>();

const emit = defineEmits<{
  (e: "addToCart", product: Product): void;
}>();

const shortcuts = useProductShortcutsStore();

function addToCart(product: Product) {
  emit("addToCart", product);
}

function formatCurrency(value: number) {
  return new Intl.NumberFormat("it-IT", {
    style: "currency",
    currency: "EUR",
  }).format(value);
}
</script>
