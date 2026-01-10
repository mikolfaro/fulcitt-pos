<template>
  <div class="grid grid-cols-2 gap-2">
    <button class="btn btn-success" @click="emit('startCardPayment')">
      {{ $t("pos-start-payment-card-button") }}
    </button>

    <button class="btn btn-success" @click="emit('startCashPayment')">
      {{ $t("pos-start-payment-cash-button") }}
    </button>

    <button class="btn btn-outline btn-error" @click="cart.clear()">
      {{ $t('pos-clear-cart-button') }}</button>
  </div>
</template>

<script setup lang="ts">
import { useFluent } from 'fluent-vue'
import { useCartStore } from '../../stores/cartStore'

const { $t } = useFluent()
const cart = useCartStore()

const emit = defineEmits<{
  (e: 'startCardPayment'): void
  (e: 'startCashPayment'): void
}>()
</script>
