-- This file should undo anything in `up.sql`
ALTER TABLE "user"
DROP CONSTRAINT user_email_unique;
