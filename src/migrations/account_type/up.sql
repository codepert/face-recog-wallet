CREATE TABLE `account_type` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(256) NOT NULL,
  `is_admin` tinyint NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- Insert basic account types
INSERT INTO `account_type` 
    (`name`, `is_admin`)
VALUES 
    ('admin', 1),
    ('email', 0);
