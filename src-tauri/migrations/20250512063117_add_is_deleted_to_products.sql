ALTER TABLE products ADD COLUMN is_deleted BOOLEAN NOT NULL DEFAULT FALSE;
UPDATE products SET is_deleted = FALSE WHERE is_deleted IS NULL;
