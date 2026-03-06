export interface UserProfileView {
    id: string;
    email: string;
    name: string;
    created_at: string;
}

export interface StorageProfileView {
    user_id: string;
    allowed_storage: number;
    taken_storage: number;
    is_blocked: boolean;
}

export interface FolderView {
    id: string;
    parent_folder_id: string;
    name: string;
    owner_id: string;
    created_at: string;
}

export interface FileView {
    id: string,
    name: string,
    owner_id: string,
    parent_folder_id: string,
    file_type: string,
    is_deleted: boolean,
    ttl?: string,
    size: number,
    upload_status: string,
    created_at: string,
    updated_at: string
}