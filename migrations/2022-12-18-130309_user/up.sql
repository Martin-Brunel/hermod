-- Your SQL goes here
CREATE TABLE `hermod`.`user` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `email` VARCHAR(100) NOT NULL UNIQUE,
    `password` VARCHAR(256) NOT NULL,
    `roles` VARCHAR(100) NOT NULL,
    `brand_id` INT NOT NULL,
    `firstname` VARCHAR(100) NOT NULL DEFAULT '',
    `lastname` VARCHAR(100) NOT NULL DEFAULT '',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `is_deleted` TINYINT NOT NULL DEFAULT 0,
    `deleted_at` DATETIME NULL,
    `updated_at` DATETIME NULL
);

CREATE TABLE `hermod`.`brand` (
  `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(45) NOT NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `is_deleted` TINYINT NOT NULL DEFAULT 0,
  `deleted_at` DATETIME NULL,
  `updated_at` DATETIME NULL
);

ALTER TABLE `hermod`.`user` 
    ADD CONSTRAINT `fk_user_1`
    FOREIGN KEY (`brand_id`)
    REFERENCES `better_way`.`brand` (`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE;

    INSERT INTO `better_way`.`brand` (`name`) VALUES ('Orgavita');