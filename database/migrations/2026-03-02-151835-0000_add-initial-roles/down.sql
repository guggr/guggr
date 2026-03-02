-- This file should undo anything in `up.sql`
DELETE FROM "role"
WHERE
	"id" IN ('owner', 'admin', 'user');
