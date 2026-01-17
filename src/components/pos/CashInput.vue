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

  <div class="grid grid-cols-3 gap-2">
    <button v-for="digit in digits" :value="digit" class="btn" @click="type(digit)" :key="digit">
      {{ digit }}
    </button>

    <a class="btn" @click="type('.')">.</a>
    <a class="btn" @click="type(0)">0</a>
    <a class="btn" @click="undoType()">&lt;-</a>
  </div>

  <button class="btn btn-success w-full mt-4" @click="processPayment">
    {{ $t('pos-process-payment-button') }}
  </button>
  <button class="btn btn-outline btn-error w-full mt-2" @click="reopenCart">
    {{ $t('pos-reopen-cart-button') }}
  </button>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { useFluent } from 'fluent-vue';
import { useCartStore } from '../../stores/cartStore';
import { useMessagesStore } from '../../stores/messagesStore';
import { AppMessage } from '../../lib';
import { addEmitHelper } from 'typescript';

const router = useRouter()
const { $t } = useFluent()
const messages = useMessagesStore()
const cart = useCartStore()
const digits = [7, 8, 9, 4, 5, 6, 1, 2, 3]
const typedAmount = ref<string>('')
const amount = ref<number>(0);

const emit = defineEmits<{
  (e: 'processCashPayment', payedAmount: number): void,
  (e: 'reopenCart'): void
}>()

const formatCurrency = (value: number) => {
  return new Intl.NumberFormat('it-IT', { style: 'currency', currency: 'EUR' }).format(value);
}

function type(digit: number | string) {
  typedAmount.value += digit.toString()
  amount.value = parseFloat(typedAmount.value)
}

function undoType() {
  if (typedAmount.value.length > 1) {
    typedAmount.value = typedAmount.value.slice(0, -1)
    amount.value = parseFloat(typedAmount.value)
  } else {
    typedAmount.value = ''
    amount.value = 0
  }
}

function processPayment() {
  if (amount.value < cart.total) {
    messages.addMessage({ type: 'InvalidInput', message: $t('pos-messages-insufficient-amount') })
  } else {
    emit('processCashPayment', amount.value)
  }
}

function reopenCart() {
  emit('reopenCart')
}
</script>
