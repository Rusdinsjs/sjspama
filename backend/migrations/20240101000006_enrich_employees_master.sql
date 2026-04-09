-- Add comprehensive fields to employees table based on SJS-PAMA master data
ALTER TABLE employees ADD COLUMN contract_type VARCHAR(50);
ALTER TABLE employees ADD COLUMN ktp_number VARCHAR(20);
ALTER TABLE employees ADD COLUMN full_address TEXT;
ALTER TABLE employees ADD COLUMN village VARCHAR(100);
ALTER TABLE employees ADD COLUMN district VARCHAR(100);
ALTER TABLE employees ADD COLUMN city VARCHAR(100);
ALTER TABLE employees ADD COLUMN province VARCHAR(100);
ALTER TABLE employees ADD COLUMN origin_status VARCHAR(50);
ALTER TABLE employees ADD COLUMN gender VARCHAR(20);
ALTER TABLE employees ADD COLUMN marital_status VARCHAR(10);
ALTER TABLE employees ADD COLUMN religion VARCHAR(50);
ALTER TABLE employees ADD COLUMN birth_place VARCHAR(100);
ALTER TABLE employees ADD COLUMN birth_date DATE;
ALTER TABLE employees ADD COLUMN education VARCHAR(255);
ALTER TABLE employees ADD COLUMN email VARCHAR(255);
ALTER TABLE employees ADD COLUMN simper_number VARCHAR(100);
ALTER TABLE employees ADD COLUMN simper_expiry DATE;
