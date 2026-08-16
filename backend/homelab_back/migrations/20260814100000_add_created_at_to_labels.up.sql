-- The Labels screen lists a creation date per label; the table has carried no
-- timestamp until now. Pre-existing labels adopt the migration time.
ALTER TABLE labels
    ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
