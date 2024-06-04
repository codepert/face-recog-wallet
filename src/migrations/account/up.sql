CREATE TABLE `account` (
  `id` int NOT NULL AUTO_INCREMENT,
  `identity` varchar(256) NOT NULL COMMENT 'Feature or CESS account',
  `password` varchar(256) NOT NULL COMMENT 'hashed password if feature account, random string if CESS account is used',
  `feature` varchar(256) DEFAULT NULL COMMENT 'Feature equal to identity if feature is used to register or bind feature',
  `mnemonic` varchar(256) DEFAULT NULL COMMENT 'If feature is used to register, this is generated for the user',
  `address` varchar(256) DEFAULT NULL,
  `account_type_id` int NOT NULL COMMENT 'method used to register. (Feature or CESS account)',
  `is_active` tinyint NOT NULL DEFAULT '0' COMMENT 'Account Status: 0 (Not Active) or 1 (Active), Default 0',
  `creation_time` datetime NOT NULL COMMENT 'Account creation time',
  `jwt_token` varchar(256) DEFAULT NULL,
  `is_first_login` tinyint NOT NULL DEFAULT '1' COMMENT '0 = false, 1 = true',
  PRIMARY KEY (`id`),
  UNIQUE KEY `identity_UNIQUE` (`identity`),
  UNIQUE KEY `feature_UNIQUE` (`feature`),
  UNIQUE KEY `address_UNIQUE` (`address`),
  KEY `account_account_type_id_fk_idx` (`account_type_id`),
  CONSTRAINT `account_account_type_id_fk` FOREIGN KEY (`account_type_id`) REFERENCES `account_type` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- One admin account
-- One feature user -- Update feature user with mnemonic and address
-- One wallet user

INSERT INTO `decloud`.`account`
(`identity`,`password`,`feature`,`mnemonic`,`address`,`account_type_id`,`is_active`,`creation_time`,`jwt_token`,`is_first_login`)
VALUES
(
	'admin@cess.one',
	'$2b$12$GBdkSlEN5W0IqLjLJ2LrP.a3y3uh4ZFS3uVuMnWcVjaoPq48fIytu',
	'admin_feature',
	null,
	null,
	1,
	1,
	'2023-12-08 00:00:00',
	'2023-12-08 00:00:00',
	null,
  	0
),
(
	'user1@cess.one',
	'$2b$12$GBdkSlEN5W0IqLjLJ2LrP.a3y3uh4ZFS3uVuMnWcVjaoPq48fIytu',
	'admin_feature',
	null,
	null,
	2,
	1,
	'2023-12-08 00:00:00',
	'2023-12-08 00:00:00',
	null,
  	0
),
(
	'5FCvhc9ubX1v9NQE6pAJkC62Ri18CBodBLHzo9Y71bV2cQmg',
	'$2b$12$GBdkSlEN5W0IqLjLJ2LrP.a3y3uh4ZFS3uVuMnWcVjaoPq48fIytu',
	'dragon@cess.one',
	null,
	'cXi7t4f7QZpXu7AB3DmBqCdFxcBFVMPAzroz2k8HVpnUgwXgX',
	3,
	1,
	'2023-12-08 00:00:00',
	'2023-12-08 00:00:00',
	null,
  	0
);
