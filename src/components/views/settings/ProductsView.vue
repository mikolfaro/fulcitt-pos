<template>
  <div>
    <h2 class="text-xl font-semibold mb-4">{{ $t('settings-products-title') }}</h2>
    <div class="card bg-base-200 shadow-md mt-8">
      <div class="card-body">
        <div v-if="isLoadingProducts" class="text-center">
          <span class="loading loading-dots loading-md"></span>
          {{ $t('settings-products-messages-loading-existing-products') }}
        </div>
        <div v-else class="overflow-x-auto">
          <table class="table table-zebra w-full">
            <thead>
              <tr>
                <th>{{ $t('settings-products-product-name') }}</th>
                <th>{{ $t('settings-products-category') }}</th>
                <th class="text-right">{{ $t('settings-products-price') }}</th>
                <th class="text-center">Cauzione</th>
                <th>{{ $t('settings-products-actions') }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="product in existingProducts" :key="product.id" class="hover">
                <template v-if="productToEdit.id == product.id">
                  <td>
                    <input type="text" :placeholder="$t('settings-products-product-name-example')"
                      class="input input-bordered w-full" form="editProduct" v-model.trim="productToEdit.name"
                      required />
                  </td>
                  <td>
                    <input type="text" :placeholder="$t('settings-products-category-example')"
                      class="input input-bordered w-full" form="editProduct" v-model.trim="productToEdit.category"
                      required />
                  </td>
                  <td>
                    <input type="number" step="0.01" min="0" :placeholder="$t('settings-products-price-example')"
                      class="input input-bordered w-full" form="editProduct" v-model.number="productToEdit.price"
                      required />
                  </td>
                  <td class="text-center">
                    <input type="checkbox" class="checkbox checkbox-sm" form="editProduct"
                      v-model="productToEdit.has_deposit" />
                  </td>
                  <td class="flex justify-between">
                    <button type="submit" class="btn btn-xs btn-outline btn-success mr-4" form="editProduct">
                      {{ $t('settings-products-save-product-button') }}
                    </button>
                    <button type="button" class="btn btn-xs btn-outline btn-error" @click="closeEdit()">
                      {{ $t('settings-products-cancel-edit-button') }}
                    </button>
                  </td>
                </template>
                <template v-else>
                  <td>{{ product.name }}</td>
                  <td>{{ product.category }}</td>
                  <td class="text-right">{{ formatCurrency(product.price) }}</td>
                  <td class="text-center">
                    <input type="checkbox" class="checkbox checkbox-sm" :checked="product.has_deposit" disabled />
                  </td>
                  <td class="flex justify-between">
                    <button class="btn btn-xs btn-outline btn-primary" @click="openEdit(product)">
                      {{ $t('settings-products-edit-product-button') }}
                    </button>

                    <button class="btn btn-xs btn-outline btn-error" @click.prevent="doDeleteProduct(product)">
                      {{ $t('settings-products-delete-product-button') }}
                    </button>
                  </td>
                </template>
              </tr>
              <tr>
                <td>
                  <input type="text" :placeholder="$t('settings-products-product-name-example')"
                    class="input input-bordered w-full" form="addProduct" v-model.trim="newProduct.name" required />
                </td>
                <td>
                  <input type="text" :placeholder="$t('settings-products-category-example')"
                    class="input input-bordered w-full" form="addProduct" v-model.trim="newProduct.category" required />
                </td>
                <td>
                  <input type="number" step="0.01" min="0" :placeholder="$t('settings-products-price-example')"
                    class="input input-bordered w-full" form="addProduct" v-model.number="newProduct.price" required />
                </td>
                <td class="text-center">
                  <input type="checkbox" class="checkbox checkbox-sm" form="addProduct"
                    v-model="newProduct.has_deposit" />
                </td>
                <td>
                  <button type="submit" class="btn btn-xs btn-outline btn-success" form="addProduct">
                    {{ $t('settings-products-add-product-button') }}
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
          <form id="addProduct" @submit.prevent="addProduct"></form>
          <form id="editProduct" @submit.prevent="doUpdateProduct"></form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useFluent } from "fluent-vue";
import { onMounted, reactive, ref } from "vue";
import { AppMessage, Product, UnsavedProduct } from "../../../lib";
import {
  createProduct,
  deleteProduct,
  listProducts,
  updateProduct,
} from "../../../repositories";
import { useMessagesStore } from "../../../stores/messagesStore";

const { $t } = useFluent();
const messages = useMessagesStore();
const isAdding = ref(false);
const isUpdating = ref(false);
const isLoadingProducts = ref(true);

const existingProducts = ref<Product[]>([]);

const newProduct = reactive<{
  name: string;
  price: number | null;
  category: string;
  has_deposit: boolean;
}>({ name: "", category: "", price: null, has_deposit: false });

const productToEdit = reactive<{
  id: number | null;
  name: string;
  price: number | null;
  category: string;
  has_deposit: boolean;
}>({ id: null, name: "", category: "", price: null, has_deposit: false });

const formatCurrency = (value: number) => {
  return new Intl.NumberFormat("it-IT", {
    style: "currency",
    currency: "EUR",
  }).format(value);
};

onMounted(async () => {
  try {
    await fetchExistingProducts();
  } catch (err) {
    messages.addMessage(err as AppMessage);
  }
});

const addProduct = async () => {
  if (newProduct.price === null || newProduct.price < 0) {
    messages.addInvalidInput(
      $t("settings-products-messages-price-must-be-positive"),
    );
    return;
  }

  isAdding.value = true;

  try {
    const product = newProduct as UnsavedProduct;
    await createProduct(product);
    await fetchExistingProducts();

    newProduct.name = "";
    newProduct.price = null;
    newProduct.category = "";
    newProduct.has_deposit = false;
  } catch (err: unknown) {
    messages.addMessage(err as AppMessage);
  } finally {
    isAdding.value = false;
  }
};

const fetchExistingProducts = async () => {
  isLoadingProducts.value = true;
  try {
    const products = await listProducts();
    existingProducts.value = products.sort((a, b) => a.category.localeCompare(b.category, "it") || a.name.localeCompare(b.name, "it"));
  } catch (err: unknown) {
    console.error("Failed to fetch products", err);
    messages.addMessage({
      type: "DataLoading",
      message: `Errore durante il caricamento dei prodotti`,
    });
    existingProducts.value = [];
  } finally {
    isLoadingProducts.value = false;
  }
};

const resetEditForm = () => {
  Object.assign(productToEdit, {
    id: null,
    name: "",
    price: null,
    category: "",
    has_deposit: false,
  });
};

const openEdit = (product: Product) => {
  resetEditForm();
  Object.assign(productToEdit, product);
};

const closeEdit = () => {
  resetEditForm();
  Object.assign(productToEdit, {
    id: null,
    name: "",
    category: "",
    price: null,
    has_deposit: false,
  });
};

const doUpdateProduct = async () => {
  if (
    !productToEdit.name ||
    productToEdit.price === null ||
    productToEdit.price < 0 ||
    !productToEdit.category
  ) {
    messages.addInvalidInput($t("settings-products-messages-incorrect-fields"));
    return;
  }
  const product = productToEdit as Product;

  isUpdating.value = true;

  try {
    await updateProduct(product);
    messages.addSuccess($t("settings-products-messages-product-updated"));
    await fetchExistingProducts();
    closeEdit();
  } catch (err: unknown) {
    console.error(`Error updating product ID ${productToEdit.id}:`, err);
    messages.addMessage(err as AppMessage);
  } finally {
    isUpdating.value = false;
  }
};

const doDeleteProduct = async (productToDelete: Product) => {
  try {
    await deleteProduct(productToDelete);
    messages.addSuccess($t("settings-products-messages-product-deleted"));
    await fetchExistingProducts();
  } catch (err: unknown) {
    console.error(`Error deleting product ID ${productToDelete.id}:`, err);
    messages.addMessage(err as AppMessage);
  }
};
</script>
