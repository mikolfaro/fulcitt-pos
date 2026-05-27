/**
 * Keyboard shortcut configuration for the POS view.
 * Change the key values here to remap shortcuts globally.
 *
 * Key values follow the KeyboardEvent.key spec:
 * https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values
 */
export const POS_SHORTCUTS = {
  /** Trigger card payment */
  CARD_PAYMENT: "F1",
  /** Trigger cash payment (open cash input screen) */
  CASH_PAYMENT: "F2",
  /** Cancel / clear cart */
  CANCEL_ORDER: "Escape",
  /** Reprint last sale ticket */
  REPRINT: "F3",
  /** Open cash drawer */
  OPEN_DRAWER: "F4",
} as const;

/** Shortcuts active on the cash input screen. */
export const CASH_INPUT_SHORTCUTS = {
  /** Confirm and process the cash payment */
  CONFIRM: "F2",
  /** Go back to order recap without clearing the cart */
  REOPEN: "Escape",
  BACKSPACE: "Backspace",
  /** Enter also confirms, for convenience */
  CONFIRM_ALT: "Enter",
} as const;
