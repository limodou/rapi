-- Add migration script here
ALTER TABLE `group` 
ADD UNIQUE INDEX `idx_group_title_UNIQUE` (`group_title` ASC);