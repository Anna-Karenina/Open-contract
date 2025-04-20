-- This file should undo anything in `up.sql`

ALTER TABLE projects
DROP COLUMN project_link
DROP COLUMN description
DROP COLUMN creator_id
DROP COLUMN updated_at
