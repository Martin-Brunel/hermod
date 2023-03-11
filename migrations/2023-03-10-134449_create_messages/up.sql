-- Your SQL goes here
CREATE TABLE `hermod`.`message` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `data` TEXT NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `is_deleted` TINYINT NOT NULL DEFAULT 0,
    `deleted_at` DATETIME NULL,
    `updated_at` DATETIME NULL
);


ALTER TABLE `hermod`.`message` 
    ADD CONSTRAINT `fk_message_1`
    FOREIGN KEY (`user_id`)
    REFERENCES `hermod`.`user` (`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE;