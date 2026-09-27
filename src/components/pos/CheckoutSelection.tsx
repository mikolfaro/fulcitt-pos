import { useFluent } from "fluent-vue";
import { defineComponent } from "vue";
import { POS_SHORTCUTS as SHORTCUTS } from "../../config/shortcuts";

export default defineComponent({
  name: "CheckoutSelection",
  emits: {
    startCardPayment: () => true,
    startCashPayment: () => true,
    cancelOrder: () => true,
  },
  setup(_props, { emit }) {
    const { $t } = useFluent();

    return () => (
      <div class="grid grid-cols-2 gap-2">
        <button
          class="btn btn-success"
          onClick={() => emit("startCardPayment")}
        >
          {$t("pos-start-payment-card-button")}
          <kbd class="kbd kbd-sm opacity-60">{SHORTCUTS.CARD_PAYMENT}</kbd>
        </button>

        <button
          class="btn btn-success"
          onClick={() => emit("startCashPayment")}
        >
          {$t("pos-start-payment-cash-button")}
          <kbd class="kbd kbd-sm opacity-60">{SHORTCUTS.CASH_PAYMENT}</kbd>
        </button>

        <button
          class="btn btn-outline btn-error"
          onClick={() => emit("cancelOrder")}
        >
          {$t("pos-clear-cart-button")}
          <kbd class="kbd kbd-sm opacity-60">{SHORTCUTS.CANCEL_ORDER}</kbd>
        </button>
      </div>
    );
  },
});
