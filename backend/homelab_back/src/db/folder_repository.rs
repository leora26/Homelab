use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::folder::Folder;
use crate::domain::user::Role::User;

pub async fn find_root_folder(user_id: &Uuid, pool: &PgPool)
                              -> Result<Option<Folder>, sqlx::Error> {
    let folder = sqlx::query_as!(
        Folder,
        "SELECT id, parent_folder_id, name, owner_id, created_at FROM folders WHERE parent_folder_id IS NULL AND owner_id = $1",
        &user_id
    )
        .fetch_optional(pool)
        .await?;

    Ok(folder)
}

pub async fn find_folder_by_id(folder_id: &Uuid, pool: &PgPool)
    -> Result<Option<Folder>, sqlx::Error> {
    let folder = sqlx::query_as!(
        Folder,
        "SELECT id, parent_folder_id, name, owner_id, created_at FROM folders WHERE id = $1",
        folder_id
    )
        .fetch_optional(pool)
        .await?;

    Ok(folder)
}

pub async fn find_all_children_folders (pool: &PgPool, folder_id: &Uuid)
    -> Result<Vec<Folder>, sqlx::Error> {
    let folders = sqlx::query_as!(
        Folder,
        "SELECT id, parent_folder_id, name, owner_id, created_at FROM folders WHERE parent_folder_id = $1",
        folder_id
    )
        .fetch_all(pool)
        .await?;

    Ok(folders)
}