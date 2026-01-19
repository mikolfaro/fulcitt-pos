import { defineStore } from "pinia";
import { CartItem, Product } from "../lib";

export const useCartStore = defineStore('cart', {
  state: () => ({
    items: [] as CartItem[],
    refundItems: [] as CartItem[],
    isLocked: false
  }),
  getters: {
    subTotal(state) {
      return state.items.reduce((sum, item) => sum + item.price * item.quantity, 0)
        - state.refundItems.reduce((sum, item) => sum + item.price * item.quantity, 0);
    },
    total(state) {
      return state.items.reduce((sum, item) => sum + item.price * item.quantity, 0)
        - state.refundItems.reduce((sum, item) => sum + item.price * item.quantity, 0);
    },
    isEmpty(state) {
      return state.items.length === 0 && state.refundItems.length === 0
    }
  },
  actions: {
    addItem(newItem: Product | CartItem, quantity: number) {
      const existingItem = this.items.find(item => item.id === newItem.id);
      if (existingItem) {
        existingItem.quantity += quantity;
      } else {
        this.items.push({ ...newItem, quantity: 1 });
      }
    },
    removeItem(itemToBeRemoved: Product | CartItem, quantity?: number) {
      if (quantity) {
        const existingItem = this.items.find(item => item.id === itemToBeRemoved.id);
        if (existingItem) {
          if (existingItem.quantity > quantity) {
            existingItem.quantity -= quantity
          } else {
            this.items = this.items.filter(item => item.id !== itemToBeRemoved.id)
          }
        }
      } else {
        this.items = this.items.filter(item => item.id !== itemToBeRemoved.id)
      }
    },
    addRefundItem(newItem: Product | CartItem, quantity: number) {
      const existingRefundItem = this.refundItems.find(item => item.id === newItem.id);
      if (existingRefundItem) {
        existingRefundItem.quantity += quantity;
      } else {
        this.refundItems.push({ ...newItem, quantity: 1 });
      }
    },
    removeRefundItem(itemToBeRemoved: Product | CartItem, quantity?: number) {
      if (quantity) {
        const existingRefundItem = this.refundItems.find(item => item.id === itemToBeRemoved.id);
        if (existingRefundItem) {
          if (existingRefundItem.quantity > quantity) {
            existingRefundItem.quantity -= quantity
          } else {
            this.refundItems = this.refundItems.filter(item => item.id !== itemToBeRemoved.id)
          }
        }
      } else {
        this.refundItems = this.refundItems.filter(item => item.id !== itemToBeRemoved.id)
      }
    },
    clear() {
      this.items = []
      this.refundItems = []
    },
    lock() {
      this.isLocked = true
    },
    unlock() {
      this.isLocked = false
    },
  }
});
