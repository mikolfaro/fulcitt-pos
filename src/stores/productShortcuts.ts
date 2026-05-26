import { load as loadStore } from "@tauri-apps/plugin-store";
import { defineStore } from "pinia";

const STORE_KEY = "product-shortcuts";

export const useProductShortcutsStore = defineStore("productShortcuts", {
  state: () => ({
    /** productId (as string) → single lowercase key character */
    shortcuts: {} as Record<string, string>,
  }),

  getters: {
    /** Reverse map: key char → productId */
    keyToProductId(state): Record<string, string> {
      const map: Record<string, string> = {};
      for (const [productId, key] of Object.entries(state.shortcuts)) {
        map[key] = productId;
      }
      return map;
    },
  },

  actions: {
    async load() {
      const store = await loadStore("store.json", { autoSave: false, defaults: {} });
      const saved = await store.get<Record<string, string>>(STORE_KEY);
      if (saved) this.shortcuts = saved;
    },

    async setShortcut(productId: string | number, key: string) {
      this.shortcuts[String(productId)] = key.toLowerCase();
      await this._persist();
    },

    async removeShortcut(productId: string | number) {
      delete this.shortcuts[String(productId)];
      await this._persist();
    },

    /** Returns the productId assigned to this key, or undefined */
    productIdForKey(key: string): string | undefined {
      return this.keyToProductId[key.toLowerCase()];
    },

    /** Returns the shortcut key for a product, or undefined */
    keyForProduct(productId: string | number): string | undefined {
      return this.shortcuts[String(productId)];
    },

    async _persist() {
      const store = await loadStore("store.json", { autoSave: false, defaults: {} });
      await store.set(STORE_KEY, this.shortcuts);
      await store.save();
    },
  },
});
