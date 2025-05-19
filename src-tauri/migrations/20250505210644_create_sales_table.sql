CREATE TABLE IF NOT EXISTS sales (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  sale_time TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now', 'utc')),
  total_amount REAL NOT NULL CHECK(total_amount >= 0),
  payment_method TEXT
);
