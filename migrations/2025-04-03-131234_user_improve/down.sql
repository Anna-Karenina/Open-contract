-- This file should undo anything in `up.sql`

ALTER TABLE users DROP COLUMN password_hash;
ALTER TABLE users DROP COLUMN is_active;
