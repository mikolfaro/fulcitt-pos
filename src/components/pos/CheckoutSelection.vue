<template>
  <div class="grid grid-cols-2 gap-2">
    <button class="btn btn-success" @click="emit('startCardPayment')">
      {{ $t("pos-start-payment-card-button") }}
      <kbd class="kbd kbd-sm opacity-60">{{ SHORTCUTS.CARD_PAYMENT }}</kbd>
    </button>

    <button class="btn btn-success" @click="emit('startCashPayment')">
      {{ $t("pos-start-payment-cash-button") }}
      <kbd class="kbd kbd-sm opacity-60">{{ SHORTCUTS.CASH_PAYMENT }}</kbd>
    </button>

    <button class="btn btn-outline btn-error" @click="emit('cancelOrder')">
      {{ $t('pos-clear-cart-button') }}
      <kbd class="kbd kbd-sm opacity-60">{{ SHORTCUTS.CANCEL_ORDER }}</kbd>
    </button>
  </div>
</template>

<script setup lang="ts">
import { useFluent } from "fluent-vue";
import { POS_SHORTCUTS as SHORTCUTS } from "../../config/shortcuts";

const { $t } = useFluent();

const emit = defineEmits<{
  (e: "startCardPayment"): void;
  (e: "startCashPayment"): void;
  (e: "cancelOrder"): void;
}>();
</script>
