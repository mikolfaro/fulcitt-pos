<template>
  <div v-if="isLoading" class="flex justify-center items-center h-full p-10">
    <div class="flex flex-col items-center space-y-2">
      <span class="loading loading-spinner loading-lg"></span>
      <p class="text-lg">{{ $t('pos-messages-loading-products') }}</p>
    </div>
  </div>
  <div v-else class="flex h-full p-4 space-x-4 bg-base-200">
    <div class="w-3/5 flex flex-col gap-4">
      <div v-if="orderCreation"
        class="flex flex-col grow bg-base-100 rounded-box shadow-lg p-4 overflow-y-auto justify-between">
        <div>
          <h2 class="text-xl font-bold mb-4">{{ $t('pos-products-title') }}</h2>
          <div v-for="(productsInCategory, category) in groupedProducts" :key="category" class="mb-6">
            <h3 class="text-lg font-semibold mb-3 sticky top-0 bg-base-100 py-1">{{ category }}</h3>

            <ProductsList :products="productsInCategory" :allowAddToCart="!cart.isLocked"
              :refundModeActive="refundModeActive" @addToCart="(p: Product) => addToCart(p)" />
          </div>
        </div>

        <div v-if="refundableProducts.length">
          <h3 class="text-xl font-bold mb-4">{{ $t('pos-products-refunds-title') }}</h3>

          <ProductsList :products="refundableProducts" :allowAddToCart="!cart.isLocked" :refundModeActive="true"
            @addToCart="(p: Product) => addToRefundCart(p)" />
        </div>
      </div>
      <div v-else>
        <CashInput @startCardPayment="startCardPayment" @processCashPayment="processCashPayment" />
      </div>


      <div v-if="orderCreation" class="flex flex-row justify-between">
        <button class="btn btn-primary" @click="printLastSale()">
          {{ $t('pos-reprint-tickets-button') }}
        </button>

        <fieldset class="fieldset bg-base-100 border-base-300 rounded-box w-64 border p-2">
          <label class="label">
            <input type="checkbox" class="toggle toggle-error" v-model="refundModeActive" />
            {{ $t('pos-cart-toggle-refund-mode') }}
          </label>
        </fieldset>
      </div>
    </div>

    <div class="w-2/5 bg-base-100 rounded-box shadow-lg p-4 flex flex-col">
      <h2 class="text-xl font-bold mb-4">{{ $t('pos-cart-title') }}</h2>
      <div class="grow overflow-y-auto mb-4">
        <OrderRecap />
      </div>
      <div class="mt-auto pt-4 border-t border-base-300">
        <!-- div class="flex justify-between font-bold text-lg mb-2">
          <span>Subtotal:</span>
          <span>{{ formatCurrency(cart.subTotal) }}</span>
        </div -->
        <div class="flex justify-between font-bold text-lg mb-2">
          <span>{{ $t('pos-recap-total') }}</span>
          <span :class="{ 'text-secondary': cart.total < 0 }">{{ formatCurrency(cart.total) }}</span>
        </div>

        <CheckoutSelection @startCardPayment="startCardPayment" @startCashPayment="startCashPayment"
          @cancelOrder="cancelOrder" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref, computed, onMounted } from 'vue';
import { useFluent } from 'fluent-vue';

import type { AppMessage } from '../../lib';
import { CartItem, PaymentType, Product } from '../../lib';
import { listProducts } from '../../repositories';
import { useCartStore } from '../../stores/cartStore';
import { useMessagesStore } from '../../stores/messagesStore';
import ProductsList from '../ProductsList.vue';
import CheckoutSelection from '../pos/CheckoutSelection.vue';
import OrderRecap from '../pos/OrderRecap.vue';
import CashInput from '../pos/CashInput.vue';
import formatCurrency from '../../utils/formatCurrency';

const availableProducts = ref<Product[]>([]);
const refundableProducts = ref<Product[]>([]);

const { $t } = useFluent();
const cart = useCartStore();
const messages = useMessagesStore()
const isLoading = ref(true);
const refundModeActive = ref<boolean>(false)
const orderCreation = ref<boolean>(true)

onMounted(async () => {
  try {
    await fetchProducts();
  } catch (err) {
    console.error("Database initialization/loading failed:", err);
    messages.addUnknownError(err)
  } finally {
    isLoading.value = false;
  }
});

// --- Methods ---
const fetchProducts = async () => {
  try {
    const products = await listProducts();
    products.sort((a: Product, b: Product) => {
      if (a.name < b.name) {
        return -1
      } else if (a.name > b.name) {
        return 1
      } else {
        return 0
      }
    })

    availableProducts.value = products;
    refundableProducts.value = products.filter((product) => product.has_deposit)
  } catch (err) {
    console.error("Error fetching products:", err);
    messages.addUnknownError(err)
  }
};

const addToCart = (product: Product) => {
  if (refundModeActive.value) {
    const item: CartItem = { id: product.id, name: product.name, price: product.price, quantity: 1, hasDeposit: product.has_deposit }

    cart.addRefundItem(item, 1);
  } else {
    cart.addItem(product, 1);
  }
};

const addToRefundCart = (product: Product) => {
  const item: CartItem = { id: product.id, name: product.name, price: product.price, quantity: 1, hasDeposit: product.has_deposit }

  cart.addRefundItem(item, 1);
};

const printLastSale = async () => {
  try {
    await invoke('print_last_sale')
  } catch (err) {
    messages.addUnknownError(err)
  }
}

const groupedProducts = computed<Record<string, Product[]>>(() => {
  const sortedProducts = availableProducts.value

  return sortedProducts.reduce((groups: Record<string, Product[]>, product: Product) => {
    const category = product.category || 'Uncategorized'; // Default category if none provided
    if (!groups[category]) {
      groups[category] = [];
    }
    groups[category].push(product);
    return groups;
  }, {});
});

const startCardPayment = async function () {
  await processPayment(PaymentType.Card)
}

const startCashPayment = async function () {
  orderCreation.value = false
}

const cancelOrder = async function () {
  cart.clear()
  orderCreation.value = true
}

const processCashPayment = async function () {
  await processPayment(PaymentType.Cash)
}

const processPayment = async function (paymentType: PaymentType) {
  if (cart.isEmpty) {
    messages.addInvalidInput($t('pos-messages-cannot-process-sale-with-no-items'))
    return;
  }

  try {
    const saleItems = cart.items.map((item) => {
      return { ...item, product_id: item.id }
    })

    const refundItems = cart.refundItems.map((item) => {
      return { ...item, product_id: item.id }
    })

    await invoke('process_transaction', { saleItems, refundItems, paymentType })

    cart.clear()
    cart.unlock()

    orderCreation.value = true
  } catch (err) {
    messages.addMessage(err as AppMessage)
  }
}
</script>

<style scoped>
.table td,
.table th {
  white-space: nowrap;
}
</style>
