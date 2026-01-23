<template>
  <div class="grid grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
    <button v-for="product in products" :aria-disabled="allowAddToCart" :key="product.id"
      class="card bg-base-300 shadow-md hover:shadow-lg hover:bg-base-200 transition-shadow duration-200 ease-in-out cursor-pointer"
      :class="{ 'text-secondary': refundModeActive }" @click="() => { allowAddToCart && addToCart(product) }">
      <div class="card-body items-center justify-between text-center p-3">
        <h3 class="card-title text-sm leading-tight">{{ product.name }}</h3>
        <p class="text-md font-semibold mt-1">{{ formatCurrency(product.price) }}</p>
      </div>
    </button>
  </div>
</template>

<script setup lang="ts">
import { Product } from '../lib'

defineProps<{
  products: Product[],
  allowAddToCart: boolean,
  refundModeActive: boolean
}>()

const emit = defineEmits<{
  (e: 'addToCart', product: Product): void
}>()

function addToCart(product: Product) {
  emit('addToCart', product)
}

function formatCurrency(value: number) {
  return new Intl.NumberFormat('it-IT', { style: 'currency', currency: 'EUR' }).format(value);
}
</script>
