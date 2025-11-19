CREATE TABLE IF NOT EXISTS refund_items (
  id TEXT NOT NULL PRIMARY KEY DEFAULT(uuid()),
  refund_id TEXT NOT NULL,
  product_id TEXT NOT NULL,
  product_name TEXT NOT NULL,
  quantity INTEGER NOT NULL CHECK(quantity > 0),
  price_at_refund REAL NOT NULL CHECK(price_at_refund >= 0),
  FOREIGN KEY (refund_id) REFERENCES refunds (id) ON DELETE CASCADE,
  FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE SET NULL
);
