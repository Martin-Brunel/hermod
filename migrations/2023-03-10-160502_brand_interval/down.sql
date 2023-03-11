-- This file should undo anything in `up.sql`
ALTER TABLE `hermod`.`brand` 
REMOVE COLUMN `interval` INT NULL AFTER `updated_at`;