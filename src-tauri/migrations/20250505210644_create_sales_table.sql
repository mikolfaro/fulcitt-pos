CREATE TABLE IF NOT EXISTS sales (
  id TEXT NOT NULL PRIMARY KEY DEFAULT(uuid()),
  sale_time DATETIME NOT NULL,
  total_amount REAL NOT NULL CHECK(total_amount >= 0),
  payment_method TEXT,
  pos_name TEXT
);
