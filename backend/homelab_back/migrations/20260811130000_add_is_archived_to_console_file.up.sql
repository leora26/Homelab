-- Track whether each file version was archived (compressed) at the time of the event.
ALTER TABLE console_file
    ADD COLUMN is_archived BOOLEAN NOT NULL DEFAULT FALSE;
