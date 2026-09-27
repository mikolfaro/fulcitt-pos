import { defineComponent, type PropType } from "vue";
import type { Product } from "../lib";
import { useProductShortcutsStore } from "../stores/productShortcuts";

export default defineComponent({
  name: "ProductsList",
  props: {
    products: { type: Array as PropType<Product[]>, required: true },
    allowAddToCart: { type: Boolean, required: true },
    refundModeActive: { type: Boolean, required: true },
  },
  emits: {
    addToCart: (_product: Product) => true,
  },
  setup(props, { emit }) {
    const shortcuts = useProductShortcutsStore();

    function addToCart(product: Product) {
      emit("addToCart", product);
    }

    function formatCurrency(value: number) {
      return new Intl.NumberFormat("it-IT", {
        style: "currency",
        currency: "EUR",
      }).format(value);
    }

    return () => (
      <div class="grid grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
        {props.products.map((product) => (
          <button
            aria-disabled={props.allowAddToCart}
            key={product.id}
            class={[
              "card bg-base-300 shadow-md hover:shadow-lg hover:bg-base-200 transition-shadow duration-200 ease-in-out cursor-pointer",
              { "text-secondary": props.refundModeActive },
            ]}
            onClick={() => {
              props.allowAddToCart && addToCart(product);
            }}
          >
            <div class="flex card-body justify-between items-center p-3">
              <h3 class="card-title text-sm leading-tight">{product.name}</h3>
              <div class="flex items-center gap-1 mt-1">
                <span class="text-md font-semibold">
                  {formatCurrency(product.price)}
                </span>
                {shortcuts.keyForProduct(product.id) && (
                  <kbd class="kbd kbd-xs opacity-60">
                    {shortcuts.keyForProduct(product.id)?.toUpperCase()}
                  </kbd>
                )}
              </div>
            </div>
          </button>
        ))}
      </div>
    );
  },
});
