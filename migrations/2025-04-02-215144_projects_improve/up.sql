-- Your SQL goes here
ALTER TABLE projects ALTER COLUMN creator_id SET NOT NULL;
ALTER TABLE projects ALTER COLUMN collaborators SET NOT NULL;