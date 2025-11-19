-- Add has_deposit column to products table
ALTER TABLE products ADD COLUMN has_deposit BOOLEAN NOT NULL DEFAULT false;
