-- MySQL Script generated by MySQL Workbench
-- seg 17 abr 2023 12:08:11
-- Model: New Model    Version: 1.0
-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema car_dealership
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Schema car_dealership
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `car_dealership` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci ;
USE `car_dealership` ;

-- -----------------------------------------------------
-- Table `car_dealership`.`cars_for_sale`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `car_dealership`.`cars_for_sale` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `brand` VARCHAR(50) NOT NULL,
  `model` VARCHAR(100) NOT NULL,
  `year` INT NOT NULL,
  `img` VARCHAR(255) NULL,
  `color` VARCHAR(50) NULL DEFAULT NULL,
  `mileage` INT NULL,
  `category` VARCHAR(50) NULL,
  `price` DECIMAL(10,2) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_unicode_ci;


-- -----------------------------------------------------
-- Table `car_dealership`.`customer`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `car_dealership`.`customer` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `first_name` VARCHAR(50) NOT NULL,
  `last_name` VARCHAR(50) NOT NULL,
  `birth_date` DATE NOT NULL,
  `email` VARCHAR(100) NULL DEFAULT NULL,
  `phone` VARCHAR(20) NULL DEFAULT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_unicode_ci;


-- -----------------------------------------------------
-- Table `car_dealership`.`cars_for_service`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `car_dealership`.`cars_for_service` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `customer_id` INT NOT NULL,
  `car_details` VARCHAR(160) NOT NULL,
  `mechanic` VARCHAR(160) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_unicode_ci;


-- -----------------------------------------------------
-- Table `car_dealership`.`salesperson`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `car_dealership`.`salesperson` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `first_name` VARCHAR(50) NOT NULL,
  `last_name` VARCHAR(50) NOT NULL,
  `commission` DECIMAL(4,2) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_unicode_ci;


-- -----------------------------------------------------
-- Table `car_dealership`.`invoice`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `car_dealership`.`invoice` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `customer_id` INT NOT NULL,
  `salesperson_id` INT NOT NULL,
  `car_id` INT NOT NULL,
  `amount` INT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_unicode_ci;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;

-- -----------------------------------------------------
-- Data for table `car_dealership`.`cars_for_sale`
-- -----------------------------------------------------
START TRANSACTION;
USE `car_dealership`;
INSERT INTO `car_dealership`.`cars_for_sale` (`id`, `brand`, `model`, `year`, `img`, `color`, `mileage`, `category`, `price`) VALUES (1, 'Hyundai', 'i30', 2016, '/img/cars/hyundai_i30.png', 'Azul', 0, 'hatch', 81040.00);
INSERT INTO `car_dealership`.`cars_for_sale` (`id`, `brand`, `model`, `year`, `img`, `color`, `mileage`, `category`, `price`) VALUES (2, 'Honda', 'fit', 2019, '/img/cars/honda_fit.png', 'Vermelho', 0, 'hatch', 76035.00);
INSERT INTO `car_dealership`.`cars_for_sale` (`id`, `brand`, `model`, `year`, `img`, `color`, `mileage`, `category`, `price`) VALUES (3, 'Toyota', 'yaris', 2019, '/img/cars/toyota_yaris.png', 'Branco', 0, 'hatch', 84056.00);
INSERT INTO `car_dealership`.`cars_for_sale` (`id`, `brand`, `model`, `year`, `img`, `color`, `mileage`, `category`, `price`) VALUES (4, 'Volkswagen', 'golf', 2017, '/img/cars/volkswagen_golf.png', 'Branco', 0, 'hatch', 79011.00);

COMMIT;


-- -----------------------------------------------------
-- Data for table `car_dealership`.`customer`
-- -----------------------------------------------------
START TRANSACTION;
USE `car_dealership`;
INSERT INTO `car_dealership`.`customer` (`id`, `first_name`, `last_name`, `birth_date`, `email`, `phone`) VALUES (1, 'Herbert', 'Olga', '1982-11-10', 'herbertolga@gmail.com.ca', '99727701');
INSERT INTO `car_dealership`.`customer` (`id`, `first_name`, `last_name`, `birth_date`, `email`, `phone`) VALUES (2, 'Isabela', 'Cristina', '1997-05-05', 'isabela@gmail.com', '99700522');
INSERT INTO `car_dealership`.`customer` (`id`, `first_name`, `last_name`, `birth_date`, `email`, `phone`) VALUES (3, 'Caio', 'Batista', '1990-10-06', 'caio_batista@hotmail.com', '99700113');

COMMIT;


-- -----------------------------------------------------
-- Data for table `car_dealership`.`salesperson`
-- -----------------------------------------------------
START TRANSACTION;
USE `car_dealership`;
INSERT INTO `car_dealership`.`salesperson` (`id`, `first_name`, `last_name`, `commission`) VALUES (1, 'Candido', 'Martins', 4.30);
INSERT INTO `car_dealership`.`salesperson` (`id`, `first_name`, `last_name`, `commission`) VALUES (2, 'Custodio', 'Alcantara', 3.90);
INSERT INTO `car_dealership`.`salesperson` (`id`, `first_name`, `last_name`, `commission`) VALUES (3, 'Maria', 'Menezes', 4.10);
INSERT INTO `car_dealership`.`salesperson` (`id`, `first_name`, `last_name`, `commission`) VALUES (4, 'Rodolfo', 'Zimmerman', 4.20);

COMMIT;

