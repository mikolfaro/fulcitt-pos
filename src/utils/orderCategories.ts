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
