<template>
  <div class="mt-2">
    <button class="btn btn-success w-full" @click="startPayment()">
      {{ t("pos.start_payment_button") }}
    </button>
    <button class="btn btn-outline btn-error w-full mt-2" @click="cart.clear()">
      {{ t('pos.clear_cart_button') }}</button>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useCartStore } from '../../../stores/cartStore';
import { useMessagesStore } from '../../../stores/messagesStore';
import { useI18n } from 'vue-i18n';

const { t } = useI18n()
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
