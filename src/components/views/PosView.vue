<template>
  <div v-if="isLoading" class="flex justify-center items-center h-full p-10">
    <div class="flex flex-col items-center space-y-2">
      <span class="loading loading-spinner loading-lg"></span>
      <p class="text-lg">Loading Database & Products...</p>
      <p v-if="loadingError" class="text-error text-sm">{{ loadingError }}</p>
    </div>
  </div>
  <div v-else class="flex h-full p-4 space-x-4 bg-base-200">
    <div class="w-3/5 bg-base-100 rounded-box shadow-lg p-4 overflow-y-auto">
      <h2 class="text-xl font-bold mb-4">Products</h2>
      <div v-for="(productsInCategory, category) in groupedProducts" :key="category" class="mb-6">
        <h3 class="text-lg font-semibold mb-3 sticky top-0 bg-base-100 py-1">{{ category }}</h3>
        <div class="grid grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          <div
            v-for="product in productsInCategory"
            :aria-disabled="cart.isLocked"
            :key="product.id"
            class="card bg-base-300 shadow-md hover:shadow-lg hover:bg-base-200 transition-shadow duration-200 ease-in-out cursor-pointer"
            @click="!cart.isLocked && addToCart(product)"
          >
            <div class="card-body items-center text-center p-3">
              <h3 class="card-title text-sm leading-tight">{{ product.name }}</h3>
              <p class="text-md font-semibold mt-1">${{ product.price.toFixed(2) }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="w-2/5 bg-base-100 rounded-box shadow-lg p-4 flex flex-col">
      <h2 class="text-xl font-bold mb-4">Current Order</h2>
      <div class="flex-grow overflow-y-auto mb-4">
        <table class="table table-zebra w-full">
          <thead>
            <tr>
              <th>Item</th>
              <th>Qty</th>
              <th>Price</th>
              <th>Total</th>
              <th></th> </tr>
          </thead>
          <tbody>
            <tr v-if="cart.items.length === 0">
              <td colspan="5" class="text-center">Cart is empty</td>
            </tr>
            <tr v-for="item in cart.items" :key="item.id">
              <td>{{ item.name }}</td>
              <td>
                <button class="btn btn-xs btn-ghost" @click="decrementQuantity(item)">-</button>
                {{ item.quantity }}
                <button class="btn btn-xs btn-ghost" @click="incrementQuantity(item)">+</button>
              </td>
              <td>${{ item.price.toFixed(2) }}</td>
              <td>${{ (item.price * item.quantity).toFixed(2) }}</td>
              <td>
                <button class="btn btn-xs btn-error btn-ghost" @click="removeFromCart(item)">X</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <div class="mt-auto pt-4 border-t border-base-300">
        <div class="flex justify-between font-bold text-lg mb-2">
          <span>Subtotal:</span>
          <span>${{ cart.subTotal.toFixed(2) }}</span>
        </div>
         <div class="flex justify-between font-bold text-lg mb-2">
          <span>Total:</span>
          <span>${{ cart.total.toFixed(2) }}</span>
        </div>
        <router-view></router-view>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref, computed, onMounted } from 'vue';
import { CartItem, Product } from '../../lib';
import { listProducts } from '../../repositories';
import { useCartStore } from '../../stores/cartStore';

const availableProducts = ref<Product[]>([]);
const cart = useCartStore();
const isLoading = ref(true);
const loadingError = ref('');

onMounted(async () => {
  loadingError.value = '';

  try {
    await fetchProducts();
  } catch (err: any) {
    console.error("Database initialization/loading failed:", err);
    loadingError.value = `Failed to load database: ${err.message || err}`;
  } finally {
    isLoading.value = false;
  }
});

// --- Methods ---
const fetchProducts = async () => {
  try {
    availableProducts.value = await listProducts();
  } catch (err: any) {
    console.error("Error fetching products:", err);
    loadingError.value = `Failed to fetch products: ${err.message || err}`;
  }
};

const addToCart = (product: Product) => {
  cart.addItem(product, 1);
};

const incrementQuantity = (item: CartItem) => {
  cart.addItem(item, 1)
};

const decrementQuantity = (item: CartItem) => {
  cart.removeItem(item, 1)
};

const removeFromCart = (itemToRemove: CartItem) => {
  cart.removeItem(itemToRemove)
};

const processPayment = async () => {
  if (cartItems.value.length === 0) {
    alert("Cart is empty!");
    return;
  }

  try {
    const items = cartItems.value.map((item) => {
      return { ...item, product_id: item.id }
    })
    await invoke('process_sale', { items })
    cart.clear()
  } catch (err) {
    console.error("Error processing payment:", err);
  }
};

const groupedProducts = computed<Record<string, Product[]>>(() => {
  return availableProducts.value.reduce((groups: Record<string, Product[]>, product: Product) => {
    const category = product.category || 'Uncategorized'; // Default category if none provided
    if (!groups[category]) {
      groups[category] = [];
    }
    groups[category].push(product);
    return groups;
  }, {});
 });
</script>

<style scoped>
/* Scoped styles if needed */
/* Ensure table cells don't wrap unnecessarily */
.table td, .table th {
  white-space: nowrap;
}
</style>
