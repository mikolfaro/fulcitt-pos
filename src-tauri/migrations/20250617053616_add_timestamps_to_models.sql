ALTER TABLE products
  ADD COLUMN created_at TEXT NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%S', 'NOW'));
ALTER TABLE products
  ADD COLUMN updated_at TEXT NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%S', 'NOW'));

CREATE TRIGGER update_products_updated_at
AFTER UPDATE ON products
FOR EACH ROW
BEGIN
    UPDATE products
    SET updated_at = STRFTIME('%Y-%m-%d %H:%M:%S', 'NOW')
    WHERE id = NEW.id;
END;

ALTER TABLE sales
  ADD COLUMN created_at TEXT NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%S', 'NOW'));
ALTER TABLE sales
  ADD COLUMN updated_at TEXT NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%S', 'NOW'));

CREATE TRIGGER update_sales_updated_at
AFTER UPDATE ON sales
FOR EACH ROW
BEGIN
    UPDATE sales
    SET updated_at = STRFTIME('%Y-%m-%d %H:%M:%S', 'NOW')
    WHERE id = NEW.id;
END;

ALTER TABLE sale_items
  ADD COLUMN created_at TEXT NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%S', 'NOW'));
ALTER TABLE sale_items
  ADD COLUMN updated_at TEXT NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%S', 'NOW'));

CREATE TRIGGER update_sale_items_updated_at
AFTER UPDATE ON sale_items
FOR EACH ROW
BEGIN
    UPDATE sale_items
    SET updated_at = STRFTIME('%Y-%m-%d %H:%M:%S', 'NOW')
    WHERE id = NEW.id;
END;
