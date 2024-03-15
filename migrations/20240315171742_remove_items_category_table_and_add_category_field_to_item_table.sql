-- Add migration script here
DROP TABLE itemscategory;

ALTER TABLE items ADD COLUMN category TEXT NOT NULL;