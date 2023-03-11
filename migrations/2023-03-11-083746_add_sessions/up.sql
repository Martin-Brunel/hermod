-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `hermod`.`session` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `is_deleted` TINYINT NOT NULL DEFAULT 0,
    `deleted_at` DATETIME NULL,
    `updated_at` DATETIME NULL
);

ALTER TABLE `hermod`.`message` 
ADD  COLUMN `session_id` INT NOT NULL AFTER `data`;

ALTER TABLE `hermod`.`message` 
ADD INDEX `fk_message_2_idx` (`session_id` ASC) VISIBLE;

ALTER TABLE `hermod`.`message` 
ADD CONSTRAINT `fk_message_1`
  FOREIGN KEY (`user_id`)
  REFERENCES `hermod`.`user` (`id`)
  ON DELETE NO ACTION
  ON UPDATE NO ACTION,
ADD CONSTRAINT `fk_message_2`
  FOREIGN KEY (`session_id`)
  REFERENCES `hermod`.`session` (`id`)
  ON DELETE NO ACTION
  ON UPDATE NO ACTION;