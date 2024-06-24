-- Your SQL goes here
ALTER TABLE `users` DROP COLUMN `age`;
ALTER TABLE `users` ADD COLUMN `age` INTEGER NOT NULL;

