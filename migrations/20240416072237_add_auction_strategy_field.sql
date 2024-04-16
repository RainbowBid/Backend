-- Add migration script here
ALTER TABLE auctions ADD COLUMN strategy TEXT NOT NULL DEFAULT 'standard';
