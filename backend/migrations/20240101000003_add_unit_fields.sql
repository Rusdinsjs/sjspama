-- Add new fields to units table
ALTER TABLE units ADD COLUMN hull_number VARCHAR(50);
ALTER TABLE units ADD COLUMN ct_number VARCHAR(50);
ALTER TABLE units ADD COLUMN ct_expired DATE;
ALTER TABLE units ADD COLUMN bast_number VARCHAR(50);
ALTER TABLE units ADD COLUMN skbp_pajak VARCHAR(50);
