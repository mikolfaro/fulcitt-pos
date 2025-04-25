<template>
  <div class="flex h-screen p-4 space-x-4 bg-base-200">
    <div class="w-3/5 bg-base-100 rounded-box shadow-lg p-4">
      <h2 class="text-xl font-bold mb-4">Products</h2>
      <div class="grid grid-cols-3 gap-4">
        <div v-for="product in availableProducts" :key="product.id" class="card bg-base-300 shadow-xl">
          <figure class="px-4 pt-4">
            <div class="bg-neutral-focus h-32 w-full rounded-lg flex items-center justify-center text-neutral-content">
              Image
            </div>
          </figure>
          <div class="card-body items-center text-center p-4">
            <h3 class="card-title text-sm">{{ product.name }}</h3>
            <p class="text-lg font-semibold">${{ product.price.toFixed(2) }}</p>
            <div class="card-actions">
              <button class="btn btn-primary btn-sm" @click="addToCart(product)">Add</button>
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
        <button class="btn btn-success w-full" @click="processPayment">Process Payment</button>
         <button class="btn btn-outline btn-error w-full mt-2" @click="clearCart">Clear Cart</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

// --- Reactive Data ---
const availableProducts = ref([
  { id: 1, name: 'Coffee', price: 3.50 },
  { id: 2, name: 'Tea', price: 2.80 },
  { id: 3, name: 'Muffin', price: 4.20 },
  { id: 4, name: 'Sandwich', price: 7.50 },
  { id: 5, name: 'Juice', price: 4.00 },
  { id: 6, name: 'Cake Slice', price: 5.50 },
]);

const cartItems = ref([]);

// --- Computed Properties ---
const cartSubtotal = computed(() => {
  return cartItems.value.reduce((sum, item) => sum + item.price * item.quantity, 0);
});

// Add tax/discount logic here later if needed
const cartTotal = computed(() => {
  return cartSubtotal.value; // For now, total is the same as subtotal
});


// --- Methods ---
const addToCart = (product) => {
  const existingItem = cartItems.value.find(item => item.id === product.id);
  if (existingItem) {
    existingItem.quantity++;
  } else {
    cartItems.value.push({ ...product, quantity: 1 });
  }
  console.log('Cart:', cartItems.value); // For debugging
};

const incrementQuantity = (item) => {
  item.quantity++;
   console.log('Cart:', cartItems.value); // For debugging
};

const decrementQuantity = (item) => {
  if (item.quantity > 1) {
    item.quantity--;
  } else {
    // If quantity is 1, remove the item
    removeFromCart(item);
  }
   console.log('Cart:', cartItems.value); // For debugging
};

const removeFromCart = (itemToRemove) => {
  cartItems.value = cartItems.value.filter(item => item.id !== itemToRemove.id);
   console.log('Cart:', cartItems.value); // For debugging
};

const clearCart = () => {
    cartItems.value = [];
    console.log('Cart cleared');
}

const processPayment = () => {
    // Placeholder for payment processing logic
    // This will eventually trigger invoice printing etc.
    if (cartItems.value.length === 0) {
        alert("Cart is empty!");
        return;
    }
    alert(`Processing payment for $${cartTotal.value.toFixed(2)}`);
    // TODO: Implement actual payment logic and post-payment actions (printing, history)
    // clearCart(); // Optionally clear cart after successful payment
};

</script>
