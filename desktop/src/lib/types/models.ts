/*
 * View models returned by the Tauri command layer.
 *
 * These mirror the `*View` structs in `src-tauri/src/types/model.rs` — see §C1 of
 * REDESIGN_CONTRACT.md. Timestamps are unix seconds, not display strings, so the UI can
 * sort, filter and format them; `null` means the server had no value.
 */

export interface LabelView {
    id: string;
    name: string;
    color: string;
    /** Files carrying this label. Zero when the label is embedded in a FileView. */
    file_count: number;
    created_at: number | null;
}

export interface FileView {
    id: string;
    name: string;
    owner_id: string;
    parent_folder_id: string;
    file_type: string;
    is_deleted: boolean;
    size: number;
    upload_status: string;
    ttl: number | null;
    created_at: number | null;
    updated_at: number | null;
    deleted_at: number | null;
    /** Populated by `get_files_for_folder`; empty from every other command. */
    labels: LabelView[];
}

export interface FolderView {
    id: string;
    parent_folder_id: string | null;
    name: string;
    owner_id: string;
    created_at: number | null;
    deleted_at: number | null;
}

export interface GlobalFileView {
    id: string;
    original_id: string;
    file: FileView;
    owner_name: string;
    shared_at: number | null;
}

export interface UserProfileView {
    id: string;
    email: string;
    name: string;
    role: string;
    created_at: number | null;
}

export interface StorageProfileView {
    user_id: string;
    allowed_storage: number;
    taken_storage: number;
    is_blocked: boolean;
}

export interface StorageStatsView {
    file_count: number;
    folder_count: number;
    /** Top-level trashed items — matches the row count on the Trash screen. */
    trashed_item_count: number;
    /** Recursive over the whole trash tree, so "Empty trash · X" is accurate. */
    trashed_bytes: number;
    labelled_file_count: number;
    unlabelled_file_count: number;
    shared_file_count: number;
    shared_bytes: number;
}

export interface MachineInfoView {
    hostname: string;
    address: string;
    uptime_seconds: number;
    app_version: string;
}

/** Payload of the `upload_progress` event emitted while `upload_content` streams. */
export interface UploadProgress {
    file_id: string;
    bytes_sent: number;
    total_bytes: number;
}

/**
 * A row in a file table. Folders and files share the same grid on My Files and Trash,
 * so screens normalise both into this shape rather than branching per cell.
 */
export type RowKind = "file" | "folder";

export interface TableRow {
    kind: RowKind;
    id: string;
    name: string;
    /** Folders have no size of their own; the UI renders an em dash. */
    size: number | null;
    modified: number | null;
    labels: LabelView[];
    /** Present on trash rows. */
    deletedAt?: number | null;
    parentFolderId?: string | null;
    file?: FileView;
    folder?: FolderView;
}
