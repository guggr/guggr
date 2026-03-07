-- Your SQL goes here
ALTER TABLE job_details_http
DROP CONSTRAINT job_details_http_id_fkey;

ALTER TABLE job_details_http
ADD CONSTRAINT job_details_http_id_fkey FOREIGN KEY (id) REFERENCES job (id) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE job_details_ping
DROP CONSTRAINT job_details_ping_id_fkey;

ALTER TABLE job_details_ping
ADD CONSTRAINT job_details_ping_id_fkey FOREIGN KEY (id) REFERENCES job (id) ON UPDATE CASCADE ON DELETE CASCADE;
