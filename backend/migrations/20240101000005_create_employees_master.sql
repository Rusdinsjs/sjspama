-- Rename operators to employees and add more fields for Employee Master
ALTER TABLE operators RENAME TO employees;

ALTER TABLE employees ADD COLUMN position VARCHAR(100); -- Jabatan (Operator, Driver, Mechanic, etc)
ALTER TABLE employees ADD COLUMN department VARCHAR(100) DEFAULT 'Production';
ALTER TABLE employees ADD COLUMN join_date DATE;
ALTER TABLE employees ADD COLUMN phone_number VARCHAR(20);
ALTER TABLE employees ADD COLUMN emergency_contact VARCHAR(100);

-- Update existing column constraints if needed
ALTER TABLE employees ALTER COLUMN name SET NOT NULL;
