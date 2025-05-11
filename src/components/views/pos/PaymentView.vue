<template>
  <div class="flex justify-between font-bold text-lg mb-2">
    <span>Payed:</span>
    <span>${{ amount.toFixed(2) }}</span>
  </div>
  <div
    v-if="amount >= total"
    class="flex justify-between font-bold text-lg mb-4"
  >
    <span>Change:</span>
    <span>${{ (amount - total).toFixed(2) }}</span>
  </div>
  <div
    v-else
    class="flex justify-between font-bold text-lg mb-4"
  >
    <span>Due:</span>
    <span class="text-error">${{ (total - amount).toFixed(2) }}</span>
  </div>

  <div class="grid grid-cols-3 gap-2">
    <button v-for="digit in digits"
      :value="digit"
      class="btn"
      @click="type(digit)"
    >
      {{ digit }}
    </button>

    <a class="btn" @click="type('.')">.</a>
    <a class="btn" @click="type(0)">0</a>
    <a class="btn" @click="undoType()"><-</a>
  </div>

  <button class="btn btn-success w-full" @click="processPayment">Process payment</button>
  <button
    class="btn btn-outline btn-error w-full mt-2"
    @click="cancelPayment">
  >
    Cancel
  </button>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const total = 300
const digits = [7, 8, 9, 4, 5, 6, 1, 2, 3]
const typedAmount = ref<string>('')
const amount = ref<number>(0);

const emit = defineEmits<{
  (e: 'payment-cancelled', amountPaid: number, change: number): void
  (e: 'payment-cancelled'): void
}>()

function type(digit: number | string) {
  typedAmount.value += digit.toString()
  amount.value = parseFloat(typedAmount.value)
}

function undoType() {
  if (typedAmount.value.length > 1) {
    typedAmount.value = typedAmount.value.slice(0, -1)
    console.log(typedAmount.value)
    amount.value = parseFloat(typedAmount.value)
  } else {
    typedAmount.value = ''
    amount.value = 0
  }
}

function processPayment() {
  emit('payment-processed', amount.value, Math.max(0, amount.value - total));
}

function cancelPayment() {
  emit('payment-cancelled');
}
</script>
