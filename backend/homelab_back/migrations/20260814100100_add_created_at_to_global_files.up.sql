-- "Shared since" on the Global Files screen: record when a file was published as
-- global. Rows that predate this adopt the migration time.
ALTER TABLE global_files
    ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
