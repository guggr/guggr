-- This file should undo anything in `up.sql`
DROP TABLE public.job_result_ping;

DROP TABLE public.job_result_http;

ALTER TABLE public.job_runs
DROP COLUMN reachable;
