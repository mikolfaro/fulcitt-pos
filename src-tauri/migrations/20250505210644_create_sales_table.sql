CREATE TABLE IF NOT EXISTS sales (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  sale_time DATETIME NOT NULL,
  total_amount REAL NOT NULL CHECK(total_amount >= 0),
  payment_method TEXT
);
