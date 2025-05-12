<template>
  <div class="mt-2">
    <button class="btn btn-success w-full" @click="startPayment()">Payment</button>
    <button class="btn btn-outline btn-error w-full mt-2" @click="cart.clear()">Clear Cart</button>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useCartStore } from '../../../stores/cartStore';
import { useMessagesStore } from '../../../stores/messagesStore';

const router = useRouter()
const cart = useCartStore()
const messages = useMessagesStore()

function startPayment() {
  if (cart.items.length > 0) {
    cart.lock()

    router.push("/checkout")
  } else {
    messages.addInvalidInput("Cart is empty")
  }
}
</script>
