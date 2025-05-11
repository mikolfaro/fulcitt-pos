<template>
  <div class="mt-2">
    <button class="btn btn-success w-full" @click="startPayment()">Payment</button>
    <button class="btn btn-outline btn-error w-full mt-2" @click="cart.clear()">Clear Cart</button>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useCartStore } from '../../../stores/cart';

const router = useRouter()
const cart = useCartStore()

function startPayment() {
  cart.lock()

  router.push("/checkout")
}
</script>
