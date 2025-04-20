-- Your SQL goes here
ALTER TABLE contracts DROP COLUMN comment_id;
ALTER TABLE comments ADD COLUMN contract_id INTEGER REFERENCES contracts(id);