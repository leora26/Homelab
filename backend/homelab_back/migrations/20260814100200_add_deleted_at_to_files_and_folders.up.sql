-- Trash shows when each item was deleted, for both files and folders. Nullable:
-- only trashed rows carry a value, and restoring clears it again.
ALTER TABLE files
    ADD COLUMN deleted_at TIMESTAMPTZ;

ALTER TABLE folders
    ADD COLUMN deleted_at TIMESTAMPTZ;

-- Backfill existing trashed files. File::set_as_deleted stores ttl as the deletion
-- time plus the 30-day retention window, so subtracting the window recovers it.
UPDATE files
SET deleted_at = ttl - INTERVAL '30 days'
WHERE is_deleted = TRUE
  AND ttl IS NOT NULL;

-- Folders have never recorded a deletion time and there is nothing to derive one
-- from, so pre-existing trashed folders stay NULL and render as unknown.
