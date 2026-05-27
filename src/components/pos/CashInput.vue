<template>
  <div class="flex justify-between font-bold text-lg mb-2">
    <span>{{ $t('pos-recap-total') }}</span>
    <span :class="{ 'text-secondary': cart.total < 0 }">{{ formatCurrency(cart.total) }}</span>
  </div>
  <div class="flex justify-between font-bold text-lg mb-2">
    <span>{{ $t('pos-recap-amount-payed') }}</span>
    <span>{{ formatCurrency(amount) }}</span>
  </div>
  <div v-if="amount >= cart.total" class="flex justify-between font-bold text-lg mb-4">
    <span>{{ $t('pos-recap-change') }}</span>
    <span>{{ formatCurrency(amount - cart.total) }}</span>
  </div>
  <div v-else class="flex justify-between font-bold text-lg mb-4">
    <span>{{ $t('pos-recap-amount-due') }}</span>
    <span class="text-error">{{ formatCurrency(cart.total - amount) }}</span>
  </div>

  <div class="grow grid grid-cols-3 gap-2">
    <button v-for="digit in digits" :value="digit" class="btn h-full text-2xl" @click="type(digit)" :key="digit">
      {{ digit }}
    </button>

    <a class="btn h-full text-2xl" @click="type('.')">.</a>
    <a class="btn h-full text-2xl" @click="type(0)">0</a>
    <a class="btn h-full text-2xl" @click="undoType()">&lt;-</a>
  </div>

  <button class="btn btn-success w-full mt-4" @click="processPayment">
    {{ $t('pos-process-payment-button') }}
    <kbd class="kbd kbd-sm opacity-60">{{ CASH_INPUT_SHORTCUTS.CONFIRM }}</kbd>
  </button>
  <button class="btn btn-outline btn-error w-full mt-2" @click="reopenCart">
    {{ $t('pos-reopen-cart-button') }}
    <kbd class="kbd kbd-sm opacity-60">{{ CASH_INPUT_SHORTCUTS.REOPEN }}</kbd>
  </button>
</template>

<script setup lang="ts">
import { useFluent } from "fluent-vue";
import { onMounted, onUnmounted, ref } from "vue";
import { CASH_INPUT_SHORTCUTS } from "../../config/shortcuts";
import { useCartStore } from "../../stores/cartStore";

const { $t } = useFluent();
const cart = useCartStore();
const digits = [7, 8, 9, 4, 5, 6, 1, 2, 3];
const typedAmount = ref<string>("");
const amount = ref<number>(0);

const emit = defineEmits<{
  (e: "processCashPayment", payedAmount: number): void;
  (e: "reopenCart"): void;
}>();

const formatCurrency = (value: number) => {
  return new Intl.NumberFormat("it-IT", {
    style: "currency",
    currency: "EUR",
  }).format(value);
};

function type(digit: number | string) {
  typedAmount.value += digit.toString();
  amount.value = parseFloat(typedAmount.value);
}

function undoType() {
  if (typedAmount.value.length > 1) {
    typedAmount.value = typedAmount.value.slice(0, -1);
    amount.value = parseFloat(typedAmount.value);
  } else {
    typedAmount.value = "";
    amount.value = 0;
  }
}

function processPayment() {
  emit("processCashPayment", cart.total);
}

function reopenCart() {
  emit("reopenCart");
}

const handleKeyDown = (e: KeyboardEvent) => {
  // Ignore when typing in an actual input element
  const target = e.target as HTMLElement;
  if (target.tagName === "INPUT" || target.tagName === "TEXTAREA") return;

  if (
    e.key === CASH_INPUT_SHORTCUTS.CONFIRM ||
    e.key === CASH_INPUT_SHORTCUTS.CONFIRM_ALT
  ) {
    e.preventDefault();
    processPayment();
  } else if (e.key === CASH_INPUT_SHORTCUTS.REOPEN) {
    e.preventDefault();
    reopenCart();
  } else if (e.key === CASH_INPUT_SHORTCUTS.BACKSPACE) {
    e.preventDefault();
    undoType();
  } else if (/^\d$/.test(e.key)) {
    e.preventDefault();
    type(e.key);
  } else if (e.key === ".") {
    e.preventDefault();
    type(".");
  }
};

onMounted(() => window.addEventListener("keydown", handleKeyDown));
onUnmounted(() => window.removeEventListener("keydown", handleKeyDown));
</script>
