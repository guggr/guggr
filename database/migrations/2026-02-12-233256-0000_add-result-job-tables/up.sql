-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "job_result_ping" (
	"id" TEXT NOT NULL UNIQUE,
	"ip_address" INET NOT NULL,
	"latency" INTEGER NOT NULL,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "job_result_http" (
	"id" TEXT NOT NULL UNIQUE,
	"ip_address" INET NOT NULL,
	"status_code" INTEGER NOT NULL,
	"latency" INTEGER NOT NULL,
	"payload" BYTEA NOT NULL,
	PRIMARY KEY ("id")
);

ALTER TABLE public.job_runs
ADD COLUMN reachable BOOLEAN;

UPDATE public.job_runs
SET
	reachable = FALSE
WHERE
	reachable IS NULL;

ALTER TABLE public.job_runs
ALTER COLUMN reachable
SET NOT NULL;

ALTER TABLE "job_result_ping"
ADD FOREIGN KEY ("id") REFERENCES "job_runs" ("id") ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE "job_result_http"
ADD FOREIGN KEY ("id") REFERENCES "job_runs" ("id") ON UPDATE CASCADE ON DELETE CASCADE;
