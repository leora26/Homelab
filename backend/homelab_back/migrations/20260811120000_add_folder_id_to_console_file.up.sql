-- Track which folder each file version belonged to, mirroring nas File.parent_folder_id.
ALTER TABLE console_file
    ADD COLUMN folder_id UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';

-- New rows must supply a real folder_id; the default only backfills pre-existing rows.
ALTER TABLE console_file
    ALTER COLUMN folder_id DROP DEFAULT;
