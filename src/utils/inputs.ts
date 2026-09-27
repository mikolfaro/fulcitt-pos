// Helpers replacing `v-model` in TSX components: read the new value from an
// input/change event's target.

export const targetValue = (e: Event) =>
  (e.target as HTMLInputElement | HTMLSelectElement).value;

export const targetChecked = (e: Event) =>
  (e.target as HTMLInputElement).checked;

/** Like `v-model.number`, but an empty or invalid input yields `null`. */
export const targetNumber = (e: Event) => {
  const value = parseFloat((e.target as HTMLInputElement).value);
  return Number.isNaN(value) ? null : value;
};
