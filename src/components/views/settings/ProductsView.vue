<template>
  <div>
    <h2 class="text-xl font-semibold mb-4">{{ t('settings.products.title') }}</h2>
    <div class="card bg-base-200 shadow-md mt-8">
      <div class="card-body">
        <div v-if="isLoadingProducts" class="text-center">
          <span class="loading loading-dots loading-md"></span>
          {{ t('settings.products.messages.loading_existing_products') }}
        </div>
        <div v-else class="overflow-x-auto">
          <table class="table table-zebra w-full">
            <thead>
              <tr>
                <th>{{ t('settings.products.product_name') }}</th>
                <th>{{ t('settings.products.category') }}</th>
                <th class="text-right">{{ t('settings.products.price') }}</th>
                <th>{{ t('settings.products.actions') }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="product in existingProducts" :key="product.id" class="hover">
                <template v-if="productToEdit.id == product.id">
                  <td>
                    <input
                      type="text"
                      placeholder="e.g., Espresso"
                      class="input input-bordered w-full"
                      form="editProduct"
                      v-model.trim="productToEdit.name"
                      required
                    />
                  </td>
                  <td>
                    <input
                      type="text"
                      placeholder="e.g., Hot Drinks"
                      class="input input-bordered w-full"
                      form="editProduct"
                      v-model.trim="productToEdit.category"
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
                      form="editProduct"
                      v-model.number="productToEdit.price"
                      required
                    />
                  </td>
                  <td>
                    <button
                      type="submit"
                      class="btn btn-xs btn-outline btn-primary mr-4"
                      form="editProduct"
                    >
                      {{ t('settings.products.save_product_button') }}
                    </button>
                    <button type="button" class="btn btn-xs btn-outline btn-info" @click="closeEdit()">
                      {{ t('settings.products.cancel_edit_button') }}
                    </button>
                  </td>
                </template>
                <template v-else>
                  <td>{{ product.name }}</td>
                  <td>{{ product.category }}</td>
                  <td class="text-right">{{ formatCurrency(product.price) }}</td>
                  <td class="flex justify-between">
                    <button
                      class="btn btn-xs btn-outline btn-info"
                      @click="openEdit(product)"
                    >
                      {{ t('settings.products.edit_product_button') }}
                    </button>

                    <button
                      class="btn btn-xs btn-outline btn-error"
                      @click.prevent="doDeleteProduct(product)"
                    >
                      {{ t('settings.products.delete_product_button') }}
                    </button>
                  </td>
                </template>
              </tr>
              <tr>
                <td>
                  <input
                    type="text"
                    :placeholder="t('settings.products.product_name_example')"
                    class="input input-bordered w-full"
                    form="addProduct"
                    v-model.trim="newProduct.name"
                    required
                  />
                </td>
                <td>
                  <input
                    type="text"
                    :placeholder="t('settings.products.category_example')"
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
                    :placeholder="t('settings.products.price_example')"
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
                  >
                      {{ t('settings.products.add_product_button') }}
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
          <form id="addProduct" @submit.prevent="addProduct" ref="addProductFormRef"></form>
          <form id="editProduct" @submit.prevent="doUpdateProduct"></form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from "vue"
import { useI18n } from "vue-i18n"
import { AppMessage, Product, UnsavedProduct } from "../../../lib"
import { createProduct, deleteProduct, listProducts, updateProduct } from "../../../repositories"
import { useMessagesStore } from "../../../stores/messagesStore"

const { t } = useI18n()
const messages = useMessagesStore()
const isAdding = ref(false);
const isUpdating = ref(false);
const isLoadingProducts = ref(true);

const addProductFormRef = ref(null);
const existingProducts = ref<Product[]>([]);

const newProduct = reactive<{
  name: string,
  price: number | null,
  category: string
}>({ name: '', category: '', price: null})

const productToEdit = reactive<{
  id: number | null,
  name: string,
  price: number | null,
  category: string
}>({ id: null, name: '', category: '', price: null });

const formatCurrency = (value: number) => {
  return new Intl.NumberFormat('it-IT', { style: 'currency', currency: 'EUR' }).format(value);
}

onMounted(async () => {
  try {
    await fetchExistingProducts();
  } catch (err) {
    messages.addMessage(err as AppMessage)
  }
});

const addProduct = async () => {
  if (newProduct.price === null || newProduct.price < 0) {
    messages.addInvalidInput("Price must be zero or greater.");
    return;
  }

  isAdding.value = true;

  try {
    const product = newProduct as UnsavedProduct;
    await createProduct(product);
    messages.addSuccess(`Product "${newProduct.name}" added successfully!`);
    await fetchExistingProducts();

    newProduct.name = '';
    newProduct.price = null;
    newProduct.category = '';
  } catch (err: any) {
    if (err.message?.toLowerCase().includes('unique constraint failed')) {
      messages.addInvalidInput(
        `Error: A product with the name "${newProduct.name}" already exists.`
      );
    } else {
      messages.addMessage(err as AppMessage)
    }
  } finally {
    isAdding.value = false;
  }
};

const fetchExistingProducts = async () => {
  isLoadingProducts.value = true;
  try {
    const products = await listProducts();
    existingProducts.value = products;
  } catch (err: any) {
    messages.addMessage({ type: 'DataLoading', message: `Failed to fetch products: ${err.message || err}`});
    existingProducts.value = [];
  } finally {
    isLoadingProducts.value = false;
  }
};

const resetEditForm = () => {
    Object.assign(productToEdit, { id: null, name: '', price: null, category: '' });
};

const openEdit = (product: Product) => {
  resetEditForm();
  Object.assign(productToEdit, product);
}

const closeEdit = () => {
  resetEditForm();
  Object.assign(productToEdit, { id: null, name: '', category: '', price: null })
};

const doUpdateProduct = async () => {
  if (!productToEdit.name || productToEdit.price === null || productToEdit.price < 0 || !productToEdit.category) {
    messages.addInvalidInput("Please fill in all fields correctly (Price >= 0).")
    return;
  }
  const product = productToEdit as Product;

  isUpdating.value = true;

  try {
    await updateProduct(product);
    messages.addSuccess("Product updated successfully!");
    await fetchExistingProducts();
    closeEdit();

  } catch (err: any) {
    console.error(`Error updating product ID ${productToEdit.id}:`, err);
    if (err.message?.toLowerCase().includes('unique constraint failed')) {
      messages.addInvalidInput(`Error: Another product likely exists with the name "${productToEdit.name}".`);
    } else {
      messages.addMessage(err as AppMessage)
    }
  } finally {
    isUpdating.value = false;
  }
}

const doDeleteProduct = async (productToDelete: Product) => {
  try {
    await deleteProduct(productToDelete)
    messages.addSuccess("Product deleted successfully!");
    await fetchExistingProducts();
  } catch (err: any) {
    console.error(`Error deleting product ID ${productToDelete.id}:`, err);
    if (err.message?.toLowerCase().includes('unique constraint failed')) {
      messages.addInvalidInput(`Error: Another product likely exists with the name "${productToDelete.name}".`);
    } else {
      messages.addMessage(err as AppMessage);
    }
  }
}
</script>
