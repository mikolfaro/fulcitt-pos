import { load as loadStore } from "@tauri-apps/plugin-store";
import { defineStore } from "pinia";
import { orderCategories } from "../utils/orderCategories";

const STORE_KEY = "category-order";

export const useCategoryOrderStore = defineStore("categoryOrder", {
  state: () => ({
    /** Category names in the user's chosen display order */
    order: [] as string[],
  }),

  actions: {
    async load() {
      const store = await loadStore("store.json", {
        autoSave: false,
        defaults: {},
      });
      const saved = await store.get<string[]>(STORE_KEY);
      if (saved) this.order = saved;
    },

    async save(order: string[]) {
      const store = await loadStore("store.json", {
        autoSave: false,
        defaults: {},
      });
      await store.set(STORE_KEY, order);
      await store.save();
      this.order = [...order];
    },

    /** Returns the given category names in display order */
    sortCategories(names: string[]): string[] {
      return orderCategories(names, this.order);
    },
  },
});
