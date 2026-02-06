import { defineStore } from "pinia";
import { CartItem, Product } from "../lib";

export const useCartStore = defineStore("cart", {
  state: () => ({
    items: {} as Record<string, CartItem>,
    refundItems: {} as Record<string, CartItem>,
    isLocked: false,
  }),
  getters: {
    subTotal(state) {
      return (
        Object.values(state.items).reduce(
          (sum, item) => sum + item.price * item.quantity,
          0,
        ) -
        Object.values(state.refundItems).reduce(
          (sum, item) => sum + item.price * item.quantity,
          0,
        )
      );
    },
    total(state) {
      return (
        Object.values(state.items).reduce(
          (sum, item) => sum + item.price * item.quantity,
          0,
        ) -
        Object.values(state.refundItems).reduce(
          (sum, item) => sum + item.price * item.quantity,
          0,
        )
      );
    },
    isEmpty(state) {
      return (
        Object.values(state.items).length === 0 &&
        Object.values(state.refundItems).length === 0
      );
    },
  },
  actions: {
    addItem(newItem: Product | CartItem, quantity: number) {
      if (this.items[newItem.id]) {
        this.items[newItem.id].quantity += quantity;
      } else {
        this.items[newItem.id] = { ...newItem, quantity };
      }
    },
    removeItem(itemToBeRemoved: Product | CartItem, quantity?: number) {
      const qty = quantity ?? 1;

      if (this.items[itemToBeRemoved.id]) {
        if (this.items[itemToBeRemoved.id].quantity > qty) {
          this.items[itemToBeRemoved.id].quantity -= qty;
        } else {
          delete this.items[itemToBeRemoved.id];
        }
      }
    },
    addRefundItem(newItem: Product | CartItem, quantity: number) {
      if (this.refundItems[newItem.id]) {
        this.refundItems[newItem.id].quantity += quantity;
      } else {
        this.refundItems[newItem.id] = { ...newItem, quantity };
      }
    },
    removeRefundItem(itemToBeRemoved: Product | CartItem, quantity?: number) {
      const qty = quantity ?? 1;

      if (this.refundItems[itemToBeRemoved.id]) {
        if (this.refundItems[itemToBeRemoved.id].quantity > qty) {
          this.refundItems[itemToBeRemoved.id].quantity -= qty;
        } else {
          delete this.refundItems[itemToBeRemoved.id];
        }
      }
    },
    clear() {
      this.items = {};
      this.refundItems = {};
    },
    lock() {
      this.isLocked = true;
    },
    unlock() {
      this.isLocked = false;
    },
  },
});
