-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "refresh_token" (
	"jti" TEXT NOT NULL UNIQUE,
	"user_id" TEXT NOT NULL,
	"ip_address" TEXT NOT NULL,
	"user_agent" TEXT NOT NULL,
	"expires_on" TIMESTAMP NOT NULL,
	PRIMARY KEY ("jti")
);

ALTER TABLE "refresh_token"
ADD FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON UPDATE CASCADE ON DELETE CASCADE;
