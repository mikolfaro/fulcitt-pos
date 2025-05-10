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
            :key="product.id"
            class="card bg-base-300 shadow-md hover:shadow-lg hover:bg-base-200 transition-shadow duration-200 ease-in-out cursor-pointer"
            @click="addToCart(product)"
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
            <tr v-if="cartItems.length === 0">
              <td colspan="5" class="text-center">Cart is empty</td>
            </tr>
            <tr v-for="item in cartItems" :key="item.id">
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
          <span>${{ cartSubtotal.toFixed(2) }}</span>
        </div>
         <div class="flex justify-between font-bold text-lg mb-4">
          <span>Total:</span>
          <span>${{ cartTotal.toFixed(2) }}</span>
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

const availableProducts = ref<Product[]>([]);
const cartItems = ref<CartItem[]>([]);
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

const cartSubtotal = computed(() => {
  return cartItems.value.reduce((sum, item) => sum + item.price * item.quantity, 0);
});

// Add tax/discount logic here later if needed
const cartTotal = computed(() => {
  return cartSubtotal.value; // For now, total is the same as subtotal
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
  const existingItem = cartItems.value.find(item => item.id === product.id);
  if (existingItem) {
    existingItem.quantity++;
  } else {
    cartItems.value.push({ ...product, quantity: 1 });
  }
};

const incrementQuantity = (item: CartItem) => {
  item.quantity++;
};

const decrementQuantity = (item: CartItem) => {
  if (item.quantity > 1) {
    item.quantity--;
  } else {
    // If quantity is 1, remove the item
    removeFromCart(item);
  }
};

const removeFromCart = (itemToRemove: CartItem) => {
  cartItems.value = cartItems.value.filter(item => item.id !== itemToRemove.id);
};

const clearCart = () => {
  cartItems.value = [];
}

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
    clearCart()
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
