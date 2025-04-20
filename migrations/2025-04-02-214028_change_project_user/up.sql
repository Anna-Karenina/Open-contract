-- Your SQL goes here
ALTER TABLE projects ADD COLUMN creator_id INTEGER REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE projects ADD COLUMN collaborators INTEGER REFERENCES collaborators(id);

