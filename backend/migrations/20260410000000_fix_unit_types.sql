-- Fix unit types based on model prefixes
UPDATE units SET type_unit = 'Compactor' WHERE model_unit ILIKE 'SV%';
UPDATE units SET type_unit = 'Dozer' WHERE model_unit ILIKE 'D%';
UPDATE units SET type_unit = 'Excavator' WHERE model_unit ILIKE 'PC%';
