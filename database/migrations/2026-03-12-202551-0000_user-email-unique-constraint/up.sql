-- Your SQL goes here
ALTER TABLE "user"
ADD CONSTRAINT user_email_unique UNIQUE (email);
