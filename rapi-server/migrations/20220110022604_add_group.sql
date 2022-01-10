-- Add migration script here
CREATE TABLE `group` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `group_title` VARCHAR(255) NOT NULL,
  `group_desc` VARCHAR(512) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`),
  UNIQUE INDEX `idx_group` (`id` ASC));

CREATE TABLE `group_users` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `group_id` INT NOT NULL COMMENT '组 ID',
  `user_id` INT NOT NULL COMMENT '用户 ID',
  `member_type` CHAR(1) NOT NULL DEFAULT '2' COMMENT '人员类型 1 组长 2 成员',
  UNIQUE INDEX `idx_group_users_id` (`id` ASC),
  PRIMARY KEY (`id`),
  UNIQUE INDEX `idx_group_users_rel` (`group_id` ASC, `user_id` ASC));