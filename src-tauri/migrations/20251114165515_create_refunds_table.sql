CREATE TABLE IF NOT EXISTS refunds (
  id TEXT NOT NULL PRIMARY KEY DEFAULT(uuid()),
  number INTEGER,
  refund_time DATETIME NOT NULL,
  total_amount REAL NOT NULL,
  payment_method TEXT,
  pos_name TEXT
);
