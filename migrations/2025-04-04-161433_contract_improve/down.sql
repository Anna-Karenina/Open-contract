-- This file should undo anything in `up.sql`


ALTER TABLE contracts DROP COLUMN http_method;
ALTER TABLE contracts DROP COLUMN description;