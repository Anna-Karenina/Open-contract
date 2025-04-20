-- This file should undo anything in `up.sql`
ALTER TABLE projects DROP COLUMN creator_id;
ALTER TABLE projects DROP COLUMN collaborators;