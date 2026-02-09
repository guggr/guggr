-- Your SQL goes here
ALTER TABLE public.job_runs
ADD COLUMN batch_id text;

UPDATE public.job_runs
SET
	batch_id = ''
WHERE
	batch_id IS NULL;

ALTER TABLE public.job_runs
ALTER COLUMN batch_id
SET NOT NULL;
