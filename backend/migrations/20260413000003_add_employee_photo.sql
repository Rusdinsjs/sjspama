-- Add photo_url to employees
ALTER TABLE employees ADD COLUMN IF NOT EXISTS photo_url TEXT;
