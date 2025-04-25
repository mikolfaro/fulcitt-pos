<template>
  <div class="flex h-screen p-4 space-x-4 bg-base-200">
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
        { id: 1, name: 'Espresso', price: 1.50, category: 'Hot Drinks' },
        { id: 2, name: 'Cappuccino', price: 2.80, category: 'Hot Drinks' },
        { id: 3, name: 'Latte', price: 3.20, category: 'Hot Drinks' },
        { id: 10, name: 'Orange Juice', price: 4.00, category: 'Cold Drinks' },
        { id: 11, name: 'Iced Tea', price: 3.50, category: 'Cold Drinks' },
        // Food
        { id: 20, name: 'Croissant', price: 1.80, category: 'Pastries' },
        { id: 21, name: 'Muffin', price: 2.50, category: 'Pastries' },
        { id: 30, name: 'Ham & Cheese Sandwich', price: 5.50, category: 'Sandwiches' },
        { id: 31, name: 'Veggie Wrap', price: 6.00, category: 'Sandwiches' },
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

const groupedProducts = computed(() => {
  return availableProducts.value.reduce((groups, product) => {
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
