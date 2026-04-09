-- Add detailed STB and BD fields to daily_logs
ALTER TABLE daily_logs ADD COLUMN work_location VARCHAR(255);
ALTER TABLE daily_logs ADD COLUMN stb_start NUMERIC(10, 2);
ALTER TABLE daily_logs ADD COLUMN stb_stop NUMERIC(10, 2);
ALTER TABLE daily_logs ADD COLUMN stb_remarks TEXT;
ALTER TABLE daily_logs ADD COLUMN bd_start NUMERIC(10, 2);
ALTER TABLE daily_logs ADD COLUMN bd_stop NUMERIC(10, 2);
ALTER TABLE daily_logs ADD COLUMN bd_remarks TEXT;
ALTER TABLE daily_logs ADD COLUMN bd_job_desc TEXT;
ALTER TABLE daily_logs ADD COLUMN bd_status VARCHAR(50);
