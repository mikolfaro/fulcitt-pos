# Manual Category Ordering Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let the user set a custom order for category sections on the POS sales screen.

**Architecture:** A pure `orderCategories` util does the ordering (unit-tested with `bun test`). A Pinia store persists the order in Tauri's `store.json` (same pattern as `src/stores/productShortcuts.ts`). A new settings tab edits the order with ▲/▼ buttons; `PosView` renders categories in that order.

**Tech Stack:** Vue 3 TSX, Pinia, `@tauri-apps/plugin-store`, DaisyUI 5, fluent-vue, Bun test runner.

Spec: `docs/superpowers/specs/2026-10-08-category-order-design.md`

---

## File Structure

- Create `src/utils/orderCategories.ts` — pure ordering function + `UNCATEGORIZED` label.
- Create `tests/orderCategories.test.ts` — unit tests. Lives outside `src/` so `tsc --noEmit` (which includes `src/**`) doesn't need `bun:test` types.
- Create `src/stores/categoryOrder.ts` — persisted order.
- Create `src/components/views/settings/CategoriesView.tsx` — reorder UI.
- Modify `src/App.tsx` — load store at startup.
- Modify `src/routes.ts`, `src/components/views/SettingsView.tsx` — new tab.
- Modify `src/components/views/PosView.tsx` — ordered grouping.
- Modify `locales/it/app.ftl` — new strings.

---

### Task 1: `orderCategories` util

**Files:**
- Create: `src/utils/orderCategories.ts`
- Test: `tests/orderCategories.test.ts`

- [ ] **Step 1: Write the failing test**

```ts
import { describe, expect, test } from "bun:test";
import { orderCategories } from "../src/utils/orderCategories";

describe("orderCategories", () => {
  test("no saved order sorts alphabetically with Italian collation", () => {
    expect(orderCategories(["panini", "Bevande", "Àperitivi"], [])).toEqual([
      "Àperitivi",
      "Bevande",
      "panini",
    ]);
  });

  test("follows saved order", () => {
    expect(
      orderCategories(["Bevande", "Dolci", "Panini"], ["Panini", "Dolci", "Bevande"]),
    ).toEqual(["Panini", "Dolci", "Bevande"]);
  });

  test("appends unknown categories alphabetically after saved ones", () => {
    expect(
      orderCategories(["Zuppe", "Bevande", "Dolci", "Antipasti"], ["Dolci"]),
    ).toEqual(["Dolci", "Antipasti", "Bevande", "Zuppe"]);
  });

  test("skips stale saved names and deduplicates", () => {
    expect(
      orderCategories(["Bevande", "Bevande", "Dolci"], ["Vecchia", "Dolci", "Dolci"]),
    ).toEqual(["Dolci", "Bevande"]);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `bun test tests/orderCategories.test.ts`
Expected: FAIL — cannot resolve `../src/utils/orderCategories`.

- [ ] **Step 3: Write minimal implementation**

```ts
/** Label used for products without a category */
export const UNCATEGORIZED = "Uncategorized";

/**
 * Orders category names following a saved manual order. Categories missing
 * from the saved order (new or renamed) are appended alphabetically; saved
 * names no longer in use are skipped.
 */
export function orderCategories(
  names: string[],
  savedOrder: string[],
): string[] {
  const present = new Set(names);
  const ordered = [...new Set(savedOrder)].filter((name) => present.has(name));
  const known = new Set(ordered);
  const rest = [...present]
    .filter((name) => !known.has(name))
    .sort((a, b) => a.localeCompare(b, "it"));

  return [...ordered, ...rest];
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `bun test tests/orderCategories.test.ts`
Expected: 4 pass, 0 fail.

- [ ] **Step 5: Commit**

```bash
git add src/utils/orderCategories.ts tests/orderCategories.test.ts
git commit -m "Add category ordering util"
```

---

### Task 2: `categoryOrder` store + startup load

**Files:**
- Create: `src/stores/categoryOrder.ts`
- Modify: `src/App.tsx`

- [ ] **Step 1: Create the store**

```ts
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
```

- [ ] **Step 2: Load it in `src/App.tsx`**

Add import `import { useCategoryOrderStore } from "./stores/categoryOrder";`, then in `setup()`:

```ts
    const categoryOrder = useCategoryOrderStore();
```

and inside the `onBeforeMount` try block after `await productShortcuts.load();`:

```ts
        await categoryOrder.load();
```

- [ ] **Step 3: Type check**

Run: `bunx tsc --noEmit`
Expected: no errors.

- [ ] **Step 4: Commit**

```bash
git add src/stores/categoryOrder.ts src/App.tsx
git commit -m "Add persisted category order store"
```

---

### Task 3: POS uses the order

**Files:**
- Modify: `src/components/views/PosView.tsx` (`groupedProducts` ~line 174, render ~line 262)

- [ ] **Step 1: Replace `groupedProducts` computed**

Add imports:

```ts
import { useCategoryOrderStore } from "../../stores/categoryOrder";
import { UNCATEGORIZED } from "../../utils/orderCategories";
```

In `setup()` next to `productShortcuts`: `const categoryOrder = useCategoryOrderStore();`

Replace the computed with:

```ts
    const groupedProducts = computed<[string, Product[]][]>(() => {
      const groups: Record<string, Product[]> = {};
      for (const product of availableProducts.value) {
        const category = product.category || UNCATEGORIZED;
        if (!groups[category]) {
          groups[category] = [];
        }
        groups[category].push(product);
      }

      return categoryOrder
        .sortCategories(Object.keys(groups))
        .map((category) => [category, groups[category]]);
    });
```

- [ ] **Step 2: Update render loop**

Replace `{Object.entries(groupedProducts.value).map(` with `{groupedProducts.value.map(` (callback body unchanged).

- [ ] **Step 3: Type check**

Run: `bunx tsc --noEmit`
Expected: no errors.

- [ ] **Step 4: Commit**

```bash
git add src/components/views/PosView.tsx
git commit -m "Order POS categories by saved order"
```

---

### Task 4: Categories settings tab

**Files:**
- Create: `src/components/views/settings/CategoriesView.tsx`
- Modify: `src/routes.ts`, `src/components/views/SettingsView.tsx`, `locales/it/app.ftl`

- [ ] **Step 1: Add strings to `locales/it/app.ftl`** (right after `settings-menu-title = Impostazioni`, before the layout keys)

```
settings-categories-menu-title = Categorie
settings-categories-title = Ordine categorie
settings-categories-description = Ordine in cui le categorie appaiono nella schermata di vendita
settings-categories-empty = Nessuna categoria. Aggiungi prima dei prodotti.
settings-categories-move-up = Sposta su
settings-categories-move-down = Sposta giù
settings-categories-messages-order-saved = Ordine categorie salvato
```

- [ ] **Step 2: Create `CategoriesView.tsx`**

```tsx
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
```

- [ ] **Step 3: Register route in `src/routes.ts`**

Import `import CategoriesView from "./components/views/settings/CategoriesView";` and add to `children` after the products entry:

```ts
      {
        path: "categories",
        component: CategoriesView,
      },
```

- [ ] **Step 4: Add tab in `SettingsView.tsx`** (after the products entry in `subRoutes`)

```ts
      {
        name: $t("settings-categories-menu-title"),
        to: "/settings/categories",
      },
```

- [ ] **Step 5: Verify**

Run: `bun test && bunx tsc --noEmit && bun run lint`
Expected: tests pass, no type errors, biome reports no changes needed (run `bun run lint:fix` if it does, then re-check).

- [ ] **Step 6: Commit**

```bash
git add src/components/views/settings/CategoriesView.tsx src/routes.ts src/components/views/SettingsView.tsx locales/it/app.ftl
git commit -m "Add categories settings tab to reorder POS categories"
```

---

### Task 5: Final verification

- [ ] **Step 1:** `bun run build` — expected success.
- [ ] **Step 2:** Manual in `bun run tauri dev`: reorder in Settings → Categorie, POS reflects it; restart, order persists; add product with new category → appears last; switch to Prodotti tab, add a category, back to Categorie → list refreshed.
