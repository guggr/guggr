-- This file should undo anything in `up.sql`
ALTER TABLE public.job_runs
DROP COLUMN batch_id;
