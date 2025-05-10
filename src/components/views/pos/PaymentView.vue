<template>
  <div class="flex justify-between font-bold text-lg mb-2">
    <span>Payed:</span>
    <span>${{ amount.toFixed(2) }}</span>
  </div>
  <div class="flex justify-between font-bold text-lg mb-4">
    <span>Rest:</span>
    <span>${{ rest.toFixed(2) }}</span>
  </div>

  <div class="grid grid-cols-3 gap-2">
    <button v-for="digit in digits"
      :value="digit"
      class="btn"
      @click="type(digit)"
    >
      {{ digit }}
    </button>

    <a class="btn col-span-2" @click="type(0)">0</a>
    <a class="btn" @click="type('.')">.</a>
  </div>

  <button class="btn btn-success w-full" @click="processPayment">Process payment</button>
  <RouterLink
    class="btn btn-outline btn-error w-full mt-2"
    to="/"
  >
    Back
  </RouterLink>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { RouterLink } from 'vue-router';

const digits = [7, 8, 9, 4, 5, 6, 1, 2, 3]
const rest = 2.0;
const typedAmount = ref<string>('')
const amount = ref<number>(0);

function type(digit: number | string) {
  typedAmount.value += digit.toString()
  amount.value = parseFloat(typedAmount.value)
}
</script>
