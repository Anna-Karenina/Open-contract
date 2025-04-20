-- Your SQL goes here

ALTER TABLE users ADD COLUMN password_hash VARCHAR(128);
ALTER TABLE users ADD COLUMN is_active BOOLEAN;

UPDATE users SET is_active = false;

ALTER TABLE users ALTER COLUMN is_active SET NOT NULL;

