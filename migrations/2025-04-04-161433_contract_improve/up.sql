-- Your SQL goes here
ALTER TABLE contracts ADD COLUMN http_method VARCHAR(64);
ALTER TABLE contracts ADD COLUMN description VARCHAR(256);