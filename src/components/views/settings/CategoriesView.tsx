import { useFluent } from "fluent-vue";
import { defineComponent, onActivated, ref } from "vue";
import { listProducts } from "../../../repositories";
import { useCategoryOrderStore } from "../../../stores/categoryOrder";
import { useMessagesStore } from "../../../stores/messagesStore";
import { UNCATEGORIZED } from "../../../utils/orderCategories";

export default defineComponent({
  name: "CategoriesView",
  setup() {
    const { $t } = useFluent();
    const messages = useMessagesStore();
    const categoryOrder = useCategoryOrderStore();

    const categories = ref<string[]>([]);
    const isLoading = ref(true);

    const fetchCategories = async () => {
      isLoading.value = true;
      try {
        const products = await listProducts();
        categories.value = categoryOrder.sortCategories(
          products.map((product) => product.category || UNCATEGORIZED),
        );
      } catch (err) {
        console.error("Failed to fetch categories", err);
        messages.addUnknownError(err);
      } finally {
        isLoading.value = false;
      }
    };

    const move = async (index: number, offset: -1 | 1) => {
      const target = index + offset;
      if (target < 0 || target >= categories.value.length) return;

      const reordered = [...categories.value];
      [reordered[index], reordered[target]] = [
        reordered[target],
        reordered[index],
      ];

      try {
        await categoryOrder.save(reordered);
        categories.value = reordered;
        messages.addSuccess($t("settings-categories-messages-order-saved"));
      } catch (err) {
        console.error("Failed to save category order", err);
        messages.addUnknownError(err);
      }
    };

    // Settings tabs are kept alive: onActivated also runs on first mount,
    // and refreshes the list when products changed in another tab
    onActivated(fetchCategories);

    return () => (
      <div>
        <h2 class="text-xl font-semibold mb-2">
          {$t("settings-categories-title")}
        </h2>
        <p class="text-sm text-base-content/70 mb-4">
          {$t("settings-categories-description")}
        </p>

        {isLoading.value ? (
          <span class="loading loading-spinner"></span>
        ) : categories.value.length === 0 ? (
          <p>{$t("settings-categories-empty")}</p>
        ) : (
          <ul class="bg-base-200 rounded-box shadow-md max-w-md divide-y divide-base-300">
            {categories.value.map((category, index) => (
              <li key={category} class="flex items-center gap-2 px-4 py-2">
                <span class="grow font-medium">{category}</span>
                <button
                  type="button"
                  class="btn btn-sm btn-square"
                  disabled={index === 0}
                  title={$t("settings-categories-move-up")}
                  aria-label={$t("settings-categories-move-up")}
                  onClick={() => move(index, -1)}
                >
                  ▲
                </button>
                <button
                  type="button"
                  class="btn btn-sm btn-square"
                  disabled={index === categories.value.length - 1}
                  title={$t("settings-categories-move-down")}
                  aria-label={$t("settings-categories-move-down")}
                  onClick={() => move(index, 1)}
                >
                  ▼
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
    );
  },
});
