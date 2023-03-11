-- Your SQL goes here
ALTER TABLE `hermod`.`message` 
ADD  COLUMN `is_read` TINYINT NOT NULL DEFAULT 0 AFTER `data`;
