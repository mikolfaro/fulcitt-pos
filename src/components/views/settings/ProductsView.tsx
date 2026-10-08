import { useFluent } from "fluent-vue";
import { defineComponent, onMounted, reactive, ref } from "vue";
import type { AppMessage, Product, UnsavedProduct } from "../../../lib";
import {
  createProduct,
  deleteProduct,
  listProducts,
  updateProduct,
} from "../../../repositories";
import { useMessagesStore } from "../../../stores/messagesStore";
import { useProductShortcutsStore } from "../../../stores/productShortcuts";
import {
  targetChecked,
  targetNumber,
  targetValue,
} from "../../../utils/inputs";

export default defineComponent({
  name: "ProductsView",
  setup() {
    const { $t } = useFluent();
    const messages = useMessagesStore();
    const shortcuts = useProductShortcutsStore();

    // Draft value for the shortcut input while in edit-row mode
    const productShortcutDraft = ref("");

    const onShortcutInput = (e: Event) => {
      productShortcutDraft.value = (e.target as HTMLInputElement).value;
    };

    const saveShortcut = async (productId: number) => {
      const key = productShortcutDraft.value.trim();
      if (key) {
        await shortcuts.setShortcut(productId, key);
      } else {
        await shortcuts.removeShortcut(productId);
      }
    };

    // For view-mode rows (shortcut is editable independently of product edit)
    const onShortcutInputDirect = (_productId: number, e: Event) => {
      // Just let the input be controlled natively; save on blur
      (e.target as HTMLInputElement).value = (
        e.target as HTMLInputElement
      ).value
        .slice(0, 1)
        .toLowerCase();
    };

    const saveShortcutDirect = async (productId: number, value: string) => {
      const key = value.trim().toLowerCase();
      if (key) {
        await shortcuts.setShortcut(productId, key);
      } else {
        await shortcuts.removeShortcut(productId);
      }
    };

    /** Returns true if the given key is already used by a different product */
    const shortcutConflict = (productId: number, key: string): boolean => {
      if (!key) return false;
      const owner = shortcuts.productIdForKey(key);
      return !!owner && owner !== String(productId);
    };

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
        existingProducts.value = products.sort(
          (a, b) =>
            a.category.localeCompare(b.category, "it") ||
            a.name.localeCompare(b.name, "it"),
        );
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
      productShortcutDraft.value = shortcuts.keyForProduct(product.id) ?? "";
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
        messages.addInvalidInput(
          $t("settings-products-messages-incorrect-fields"),
        );
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
        await fetchExistingProducts();
      } catch (err: unknown) {
        console.error(`Error deleting product ID ${productToDelete.id}:`, err);
        messages.addMessage(err as AppMessage);
      }
    };

    const editRow = (product: Product) => (
      <>
        <td>
          <input
            type="text"
            placeholder={$t("settings-products-product-name-example")}
            class="input input-bordered w-full"
            form="editProduct"
            value={productToEdit.name}
            onInput={(e) => {
              productToEdit.name = targetValue(e).trim();
            }}
            required
          />
        </td>
        <td>
          <input
            type="text"
            placeholder={$t("settings-products-category-example")}
            class="input input-bordered w-full"
            form="editProduct"
            value={productToEdit.category}
            onInput={(e) => {
              productToEdit.category = targetValue(e).trim();
            }}
            required
          />
        </td>
        <td>
          <input
            type="number"
            step="0.01"
            min="0"
            placeholder={$t("settings-products-price-example")}
            class="input input-bordered w-full"
            form="editProduct"
            value={productToEdit.price ?? ""}
            onInput={(e) => {
              productToEdit.price = targetNumber(e);
            }}
            required
          />
        </td>
        <td class="text-center">
          <input
            type="checkbox"
            class="checkbox checkbox-sm"
            form="editProduct"
            checked={productToEdit.has_deposit}
            onChange={(e) => {
              productToEdit.has_deposit = targetChecked(e);
            }}
          />
        </td>
        <td class="text-center">
          <input
            type="text"
            maxlength="1"
            class={[
              "input input-bordered input-sm w-12 text-center font-mono uppercase",
              {
                "input-error": shortcutConflict(
                  product.id,
                  productShortcutDraft.value,
                ),
              },
            ]}
            value={productShortcutDraft.value}
            onFocus={() => {
              productShortcutDraft.value =
                shortcuts.keyForProduct(product.id) ?? "";
            }}
            onInput={onShortcutInput}
            onBlur={() => saveShortcut(product.id)}
            placeholder={$t("settings-products-shortcut-placeholder")}
          />
        </td>
        <td class="flex justify-between">
          <button
            type="submit"
            class="btn btn-xs btn-outline btn-success mr-4"
            form="editProduct"
          >
            {$t("settings-products-save-product-button")}
          </button>
          <button
            type="button"
            class="btn btn-xs btn-outline btn-error"
            onClick={() => closeEdit()}
          >
            {$t("settings-products-cancel-edit-button")}
          </button>
        </td>
      </>
    );

    const viewRow = (product: Product) => (
      <>
        <td>{product.name}</td>
        <td>{product.category}</td>
        <td class="text-right">{formatCurrency(product.price)}</td>
        <td class="text-center">
          <input
            type="checkbox"
            class="checkbox checkbox-sm"
            checked={product.has_deposit}
            disabled
          />
        </td>
        <td class="text-center">
          <input
            type="text"
            maxlength="1"
            class={[
              "input input-bordered input-sm w-12 text-center font-mono uppercase",
              {
                "input-error": shortcutConflict(
                  product.id,
                  shortcuts.keyForProduct(product.id) ?? "",
                ),
              },
            ]}
            value={shortcuts.keyForProduct(product.id) ?? ""}
            onInput={(e) => onShortcutInputDirect(product.id, e)}
            onBlur={(e) =>
              saveShortcutDirect(
                product.id,
                (e.target as HTMLInputElement).value,
              )
            }
            placeholder={$t("settings-products-shortcut-placeholder")}
          />
        </td>
        <td class="flex justify-between">
          <button
            class="btn btn-xs btn-outline btn-primary"
            onClick={() => openEdit(product)}
          >
            {$t("settings-products-edit-product-button")}
          </button>

          <button
            class="btn btn-xs btn-outline btn-error"
            onClick={(e) => {
              e.preventDefault();
              doDeleteProduct(product);
            }}
          >
            {$t("settings-products-delete-product-button")}
          </button>
        </td>
      </>
    );

    return () => (
      <div>
        <h2 class="text-xl font-semibold mb-4">
          {$t("settings-products-title")}
        </h2>
        <div class="card bg-base-200 shadow-md mt-8">
          <div class="card-body">
            {isLoadingProducts.value ? (
              <div class="text-center">
                <span class="loading loading-dots loading-md"></span>
                {$t("settings-products-messages-loading-existing-products")}
              </div>
            ) : (
              <div class="overflow-x-auto">
                <table class="table table-zebra w-full">
                  <thead>
                    <tr>
                      <th>{$t("settings-products-product-name")}</th>
                      <th>{$t("settings-products-category")}</th>
                      <th class="text-right">
                        {$t("settings-products-price")}
                      </th>
                      <th class="text-center">Cauzione</th>
                      <th class="text-center">
                        {$t("settings-products-shortcut")}
                      </th>
                      <th>{$t("settings-products-actions")}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {existingProducts.value.map((product) => {
                      const isEditing = productToEdit.id == product.id;
                      // Key per mode: reusing the "Edit" button DOM node as the
                      // "Save" submit button would submit the form on the same click
                      return (
                        <tr
                          key={`${product.id}-${isEditing ? "edit" : "view"}`}
                          class="hover"
                        >
                          {isEditing ? editRow(product) : viewRow(product)}
                        </tr>
                      );
                    })}
                    <tr>
                      <td>
                        <input
                          type="text"
                          placeholder={$t(
                            "settings-products-product-name-example",
                          )}
                          class="input input-bordered w-full"
                          form="addProduct"
                          value={newProduct.name}
                          onInput={(e) => {
                            newProduct.name = targetValue(e).trim();
                          }}
                          required
                        />
                      </td>
                      <td>
                        <input
                          type="text"
                          placeholder={$t("settings-products-category-example")}
                          class="input input-bordered w-full"
                          form="addProduct"
                          value={newProduct.category}
                          onInput={(e) => {
                            newProduct.category = targetValue(e).trim();
                          }}
                          required
                        />
                      </td>
                      <td>
                        <input
                          type="number"
                          step="0.01"
                          min="0"
                          placeholder={$t("settings-products-price-example")}
                          class="input input-bordered w-full"
                          form="addProduct"
                          value={newProduct.price ?? ""}
                          onInput={(e) => {
                            newProduct.price = targetNumber(e);
                          }}
                          required
                        />
                      </td>
                      <td class="text-center">
                        <input
                          type="checkbox"
                          class="checkbox checkbox-sm"
                          form="addProduct"
                          checked={newProduct.has_deposit}
                          onChange={(e) => {
                            newProduct.has_deposit = targetChecked(e);
                          }}
                        />
                      </td>
                      <td></td>
                      <td>
                        <button
                          type="submit"
                          class="btn btn-xs btn-outline btn-success"
                          form="addProduct"
                        >
                          {$t("settings-products-add-product-button")}
                        </button>
                      </td>
                    </tr>
                  </tbody>
                </table>
                <form
                  id="addProduct"
                  onSubmit={(e) => {
                    e.preventDefault();
                    addProduct();
                  }}
                ></form>
                <form
                  id="editProduct"
                  onSubmit={(e) => {
                    e.preventDefault();
                    doUpdateProduct();
                  }}
                ></form>
              </div>
            )}
          </div>
        </div>
      </div>
    );
  },
});
