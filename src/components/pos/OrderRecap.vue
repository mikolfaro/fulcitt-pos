<template>
  <table class="table table-zebra w-full">
    <thead>
      <tr>
        <th>{{ $t('pos-cart-quantity') }}</th>
        <th>{{ $t('pos-cart-product-name') }}</th>
        <th class="overflow-column">{{ $t('pos-cart-unit-price') }}</th>
        <th>{{ $t('pos-cart-total-price') }}</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      <tr v-if="cart.items.length === 0 && cart.refundItems.length === 0">
        <td colspan="5" class="text-center">
          {{ $t('pos-messages-empty-cart') }}
        </td>
      </tr>
      <tr v-for="item in cart.items" :key="item.id">
        <td>
          <div class="flex items-center gap-1 whitespace-nowrap">
            <button class="btn btn-xs btn-primary" @click="decrementQuantity(item)">-</button>
            <span class="px-1 text-center">{{ item.quantity }}</span>
            <button class="btn btn-xs btn-primary" @click="incrementQuantity(item)">+</button>
          </div>
        </td>
        <td>{{ item.name }}</td>
        <td class="overflow-column">{{ formatCurrency(item.price) }}</td>
        <td>{{ formatCurrency(item.price * item.quantity) }}</td>
        <td>
          <button class="btn btn-xs btn-error" @click="removeFromCart(item)">X</button>
        </td>
      </tr>

      <tr v-for="item in cart.refundItems" :key="item.id" class="text-secondary">
        <td>
          <div class="flex items-center gap-1 whitespace-nowrap">
            <button class="btn btn-xs btn-primary" @click="decrementRefundQuantity(item)">-</button>
            <span class="px-1 text-center">{{ item.quantity }}</span>
            <button class="btn btn-xs btn-primary" @click="incrementRefundQuantity(item)">+</button>
          </div>
        </td>
        <td>
          {{ item.name }}
          <span v-if="item.hasDeposit">({{ $t("pos-recap-refund") }})</span>
          <span v-else>({{ $t("pos-recap-cancellation") }})</span>
        </td>
        <td class="overflow-column">{{ formatCurrency(-1.0 * item.price) }}</td>
        <td>{{ formatCurrency(-1.0 * item.price * item.quantity) }}</td>
        <td>
          <button class="btn btn-xs btn-error" @click="removeRefundFromCart(item)">X</button>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<script setup lang="ts">
import { CartItem } from '../../lib';
import { useCartStore } from '../../stores/cartStore';
import formatCurrency from '../../utils/formatCurrency';

const cart = useCartStore();

const incrementQuantity = (item: CartItem) => {
  cart.addItem(item, 1)
};

const decrementQuantity = (item: CartItem) => {
  cart.removeItem(item, 1)
};

const removeFromCart = (itemToRemove: CartItem) => {
  cart.removeItem(itemToRemove)
};

const incrementRefundQuantity = (item: CartItem) => {
  cart.addRefundItem(item, 1)
};

const decrementRefundQuantity = (item: CartItem) => {
  cart.removeRefundItem(item, 1)
};

const removeRefundFromCart = (itemToRemove: CartItem) => {
  cart.removeRefundItem(itemToRemove)
};

</script>

<style scoped>
@container style(--zoom-scale: 1.5) or style(--zoom-scale: 2) {
  .overflow-column {
    display: none;
  }
}
</style>
