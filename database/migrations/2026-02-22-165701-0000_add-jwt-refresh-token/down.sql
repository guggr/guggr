-- This file should undo anything in `up.sql`
DROP TABLE public.refresh_token;

ALTER TABLE public.user
DROP COLUMN jwt_secret;

ALTER TABLE public.user
DROP COLUMN jwt_salt;
