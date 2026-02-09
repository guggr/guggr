-- Your SQL goes here
CREATE TABLE public.job_details_http (
	id text PRIMARY KEY REFERENCES public.job (id),
	url text NOT NULL
);

CREATE TABLE public.job_details_ping (
	id text PRIMARY KEY REFERENCES public.job (id),
	host text NOT NULL
);
