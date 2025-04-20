-- This file should undo anything in `up.sql`


ALTER TABLE contracts DROP ADD COLUMN comment_id INT REFERENCES comments(id);
ALTER TABLE comments DROP COLUMN contract_id;