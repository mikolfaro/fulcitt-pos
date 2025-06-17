CREATE TABLE IF NOT EXISTS sale_items (
  id TEXT NOT NULL PRIMARY KEY DEFAULT(uuid()),
  sale_id TEXT NOT NULL,
  product_id TEXT NOT NULL,
  product_name TEXT NOT NULL,
  quantity INTEGER NOT NULL CHECK(quantity > 0),
  price_at_sale REAL NOT NULL CHECK(price_at_sale >= 0),
  FOREIGN KEY (sale_id) REFERENCES sales (id) ON DELETE CASCADE,
  FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE SET NULL
);
