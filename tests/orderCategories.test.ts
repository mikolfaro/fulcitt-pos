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
      orderCategories(
        ["Bevande", "Dolci", "Panini"],
        ["Panini", "Dolci", "Bevande"],
      ),
    ).toEqual(["Panini", "Dolci", "Bevande"]);
  });

  test("appends unknown categories alphabetically after saved ones", () => {
    expect(
      orderCategories(["Zuppe", "Bevande", "Dolci", "Antipasti"], ["Dolci"]),
    ).toEqual(["Dolci", "Antipasti", "Bevande", "Zuppe"]);
  });

  test("skips stale saved names and deduplicates", () => {
    expect(
      orderCategories(
        ["Bevande", "Bevande", "Dolci"],
        ["Vecchia", "Dolci", "Dolci"],
      ),
    ).toEqual(["Dolci", "Bevande"]);
  });
});
