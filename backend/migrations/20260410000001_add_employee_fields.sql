-- Add Batch Number and Licence to employees table
ALTER TABLE employees ADD COLUMN IF NOT EXISTS batch_number TEXT;
ALTER TABLE employees ADD COLUMN IF NOT EXISTS licence TEXT;
