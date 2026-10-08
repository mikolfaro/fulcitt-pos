# Manual category ordering — design

Date: 2026-10-08

## Goal

Let the user set a custom order for product categories on the POS sales screen.
Today categories appear in the order of their alphabetically-first product (an
artifact of object key insertion order in `PosView.tsx`).

## Scope

- In scope: order of category sections in the POS grid (`PosView`).
- Out of scope: printed tickets grouped by category, Settings → Products table,
  product order within a category.

## Approach

Frontend-only. Persist the order in Tauri's `store.json` via a Pinia store,
following the existing `productShortcuts` pattern. No DB migration, no Rust
changes. Order is per device.

## Components

### `src/stores/categoryOrder.ts`

- Store key: `category-order` in `store.json`.
- State: `order: string[]` — category names in the user's chosen order.
- Actions:
  - `load()` — read the saved array; keep `[]` if absent.
  - `save(order: string[])` — replace state and persist (`store.set` + `store.save`).
- Helper/getter `sortCategories(names: string[]): string[]`:
  1. Names present in `order`, in `order`'s sequence.
  2. Remaining names (new or renamed categories), sorted with
     `localeCompare(…, "it")`, appended after.
  - Names in `order` that are not in `names` (stale) are skipped.
- Loaded at startup in `App.tsx` alongside `productShortcuts.load()`.

### `src/components/views/settings/CategoriesView.tsx`

- New child route `/settings/categories`, linked from `SettingsView`'s nav.
- On mount and on `onActivated` (settings tabs are wrapped in `<KeepAlive>`,
  so products may have changed since mount): `listProducts()` → distinct categories (empty category shown as the
  same fallback label used by `PosView`) → `sortCategories`.
- Renders a list; each row: category name, ▲ and ▼ buttons.
  - ▲ disabled on first row, ▼ disabled on last row.
- Each move swaps adjacent entries and immediately calls `save(visibleList)`,
  then shows a success toast. Saving the full visible list prunes stale names.
- Empty state message when there are no products/categories.

### `src/components/views/PosView.tsx`

- Replace `groupedProducts` (object) with an ordered array
  `Array<[category: string, products: Product[]]>`, built by grouping then
  ordering keys via `sortCategories`.
- Render loop iterates the array instead of `Object.entries`.

### Localization

New keys in `locales/it/app.ftl`: settings tab title, page title/description,
empty state, "order saved" message, move up/down button labels (aria).

## Error handling

- Load/save failures reported via `messagesStore`, as in other settings views.
- If loading the order fails, `order` stays `[]` → alphabetical fallback.

## Verification

No test framework in the repo. Verify with:

- `bun run lint` and `bun run build` pass.
- Manual (`bun run dev`): reorder categories → POS reflects it; restart app →
  order persists; create product with a new category → appears at the end;
  rename/remove a category → no errors, list consistent.
