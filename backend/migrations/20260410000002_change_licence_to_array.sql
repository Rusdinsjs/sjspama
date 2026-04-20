-- Change licence column from text to text[] for multi-select support
ALTER TABLE employees 
ALTER COLUMN licence TYPE text[] USING 
  CASE 
    WHEN licence IS NULL THEN '{}'::text[]
    WHEN licence = '' THEN '{}'::text[]
    ELSE string_to_array(licence, ',')
  END;
