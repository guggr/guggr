-- This file should undo anything in `up.sql`
DELETE FROM job_type
WHERE
	id IN ('http', 'ping');
