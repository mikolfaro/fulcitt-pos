<template>
  <div>
    <h2 class="text-xl font-semibold mb-4">Products</h2>
    <div class="card bg-base-200 shadow-md mt-8">
      <div class="card-body">
        <div v-if="isLoadingProducts" class="text-center">
          <span class="loading loading-dots loading-md"></span> Loading products...
        </div>
        <div v-else-if="existingProducts.length === 0 && !loadingError" class="text-base-content/70">
          No products found in the database.
        </div>
        <div v-else-if="loadingError" class="text-error">
          Error loading products: {{ loadingError }}
        </div>
        <div v-else class="overflow-x-auto">
          <table class="table table-zebra w-full">
            <thead>
              <tr>
                <th>Name</th>
                <th>Category</th>
                <th class="text-right">Price</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="product in existingProducts" :key="product.id" class="hover">
                <td>{{ product.name }}</td>
                <td>{{ product.category }}</td>
                <td class="text-right">${{ product.price.toFixed(2) }}</td>
                <td>
                  <button
                    class="btn btn-xs btn-outline btn-info"
                    @click="openEdit(product)"
                  >
                    Edit
                  </button>
                </td>
              </tr>
              <tr>
                <td>
                  <input
                    type="text"
                    placeholder="e.g., Espresso"
                    class="input input-bordered w-full"
                    form="addProduct"
                    v-model.trim="newProduct.name"
                    required
                  />
                </td>
                <td>
                  <input
                    type="text"
                    placeholder="e.g., Hot Drinks"
                    class="input input-bordered w-full"
                    form="addProduct"
                    v-model.trim="newProduct.category"
                    required
                  />
                </td>
                <td>
                  <input
                    type="number"
                    step="0.01"
                    min="0"
                    placeholder="e.g., 2.50"
                    class="input input-bordered w-full"
                    form="addProduct"
                    v-model.number="newProduct.price"
                    required
                  />
                </td>
                <td>
                  <button
                    type="submit"
                    class="btn btn-xs btn-outline btn-primary"
                    form="addProduct"
                    :disabled="isAdding || !dbInstance"
                  >
                    <span v-if="isAdding" class="loading loading-spinner loading-xs"></span>
                    {{ isAdding ? 'Adding...' : 'Add' }}
                  </button>
                </td>
              </tr>
              <tr>
                <td colspan="4">
                  <p v-if="feedback.message" :class="['mt-4 text-sm', feedback.isError ? 'text-error' : 'text-success']">
                    {{ feedback.message }}
                  </p>
                </td>
              </tr>
            </tbody>
          </table>
          <form id="addProduct" @submit.prevent="addProduct" ref="addProductFormRef"></form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from "vue"
import Database from "@tauri-apps/plugin-sql";
import { Product } from "../lib";

const DB_CONNECTION_STRING = "sqlite:./app.db";
const dbInstance = ref<Database | null>(null);

const isAdding = ref(false);
const isLoadingProducts = ref(true);
const loadingError = ref('');

const addProductFormRef = ref(null);
const existingProducts = ref<Product[]>([]);

const newProduct = reactive({
  name: '',
  price: null,
  category: '',
});

const feedback = reactive({
  message: '',
  isError: false,
});

onMounted(async () => {
  feedback.message = ''; // Clear any previous messages
  try {
    dbInstance.value = await Database.load(DB_CONNECTION_STRING);
    await fetchExistingProducts();
  } catch (err) {
    console.error("ProductManagement: Failed to load database:", err);
    setFeedback(`Error connecting to database: ${err.message || err}`, true, 10000); // Show longer
  }
});

const setFeedback = (msg: string, error = false, duration = 4000) => {
    feedback.message = msg;
    feedback.isError = error;
    // Automatically clear non-error messages
    if (!error) {
        setTimeout(() => {
            if (feedback.message === msg) { // Clear only if message hasn't changed
                 feedback.message = '';
            }
        }, duration);
    }
};

const addProduct = async () => {
  if (!dbInstance.value) {
    setFeedback("Database connection not ready. Please wait.", true);
    return;
  }
  // Basic validation (HTML5 'required' handles empty fields)
  if (newProduct.price === null || newProduct.price < 0) {
      setFeedback("Price must be zero or greater.", true);
      return;
  }

  isAdding.value = true;
  feedback.message = ''; // Clear previous feedback before trying

  try {
    const result = await dbInstance.value.execute(
      "INSERT INTO products (name, price, category) VALUES ($1, $2, $3)",
      [newProduct.name, newProduct.price, newProduct.category]
    );
    setFeedback(`Product "${newProduct.name}" added successfully!`, false);
    await fetchExistingProducts();

    newProduct.name = '';
    newProduct.price = null;
    newProduct.category = '';
  } catch (err) {
    if (err.message?.toLowerCase().includes('unique constraint failed')) {
        setFeedback(`Error: A product with the name "${newProduct.name}" already exists.`, true);
    } else {
        setFeedback(`Error adding product: ${err.message || err}`, true);
    }
  } finally {
    isAdding.value = false;
  }
};

const fetchExistingProducts = async () => {
  if (!dbInstance.value) {
    loadingError.value = "Database connection not ready.";
    isLoadingProducts.value = false;
    return;
  }
  isLoadingProducts.value = true;
  loadingError.value = '';
  try {
    const products = await dbInstance.value.select(
      "SELECT id, name, price, category FROM products ORDER BY category, name"
    );
    existingProducts.value = products;
  } catch (err) {
    loadingError.value = `Failed to fetch products: ${err.message || err}`;
    existingProducts.value = []; // Clear list on error
  } finally {
    isLoadingProducts.value = false;
  }
};
</script>
