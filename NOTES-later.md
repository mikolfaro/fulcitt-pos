# Issues to look at later

Found while converting `.vue` → `.tsx`. Unrelated to the conversion; behavior preserved as-is.

- **`CashInput` emits `cart.total`, not the typed amount** — `processPayment()` does `emit("processCashPayment", cart.total)`; the `amount` the cashier typed is never sent. (`PosView.processCashPayment` ignores the payload anyway.)
- **`LayoutView` save message key has capital `S`** — `$t("Settings-layout-messages-layout-saved")`; every other key is lowercase. Check it exists in `locales/it/app.ftl`.
- **`ProductsView.closeEdit()` resets twice** — calls `resetEditForm()` then does the same `Object.assign` again.
- **`ProductsView` unused state** — `isAdding` / `isUpdating` are set but never read in the template.
- **`ProductsList` `aria-disabled` inverted** — `aria-disabled={allowAddToCart}` is `true` when adding *is* allowed.
- **Cash-payments icon in `ReportView` had `strokeLinecap`/`strokeLinejoin`** — camelCase names are invalid SVG attributes, so they were ignored. Dropped them in the TSX to keep rendering identical; switch to `stroke-linecap="round"`/`stroke-linejoin="round"` like the other icons if round caps were intended.
- **`--zoom-scale` is never set** — the `@container style(--zoom-scale: …)` rule hiding `.overflow-column` (now in `style.css`) depends on it, but nothing defines it (`applyZoom` only sets `zoom-level-*` classes and font-size). The column probably never hides.
