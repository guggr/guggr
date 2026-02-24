-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "refresh_token" (
	"jti" TEXT NOT NULL UNIQUE,
	"user_id" TEXT NOT NULL,
	"expires_on" TIMESTAMP NOT NULL,
	PRIMARY KEY ("jti")
);

ALTER TABLE "refresh_token"
ADD FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE public.user
ADD COLUMN jwt_secret TEXT;

UPDATE public.user
SET
	jwt_secret = ''
WHERE
	jwt_secret IS NULL;

ALTER TABLE public.user
ALTER COLUMN jwt_secret
SET NOT NULL;

ALTER TABLE public.user
ADD COLUMN jwt_salt TEXT;

UPDATE public.user
SET
	jwt_salt = ''
WHERE
	jwt_salt IS NULL;

ALTER TABLE public.user
ALTER COLUMN jwt_salt
SET NOT NULL;
