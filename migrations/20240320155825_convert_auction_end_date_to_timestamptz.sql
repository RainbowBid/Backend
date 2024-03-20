-- Add migration script here
ALTER TABLE auctions ALTER COLUMN end_date TYPE TIMESTAMPTZ;
