-- This file should undo anything in `up.sql`
ALTER TABLE `users` DROP COLUMN `age`;
ALTER TABLE `users` ADD COLUMN `age` INTEGER;

