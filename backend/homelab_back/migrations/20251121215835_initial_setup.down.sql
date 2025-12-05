-- The order matters! We must drop tables with foreign keys first.
DROP TABLE IF EXISTS action_logs;
DROP TABLE IF EXISTS shared_file;
DROP TABLE IF EXISTS files;
DROP TABLE IF EXISTS folders;
DROP TABLE IF EXISTS white_listed_users;
DROP TABLE IF EXISTS users;


DROP TYPE IF EXISTS action_log_type;
DROP TYPE IF EXISTS file_type;
DROP TYPE IF EXISTS user_role;
DROP TYPE IF EXISTS access_type;

-- If you make a mistake or want to change a table, do not delete the Docker container.
--
-- Edit your down.sql to drop the tables/types you created in up.sql.
--
-- Run sqlx migrate revert: sqlx migrate revert
--
-- Edit your up.sql.
--
-- Run sqlx migrate run again: sqlx migrate run