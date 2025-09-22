use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::file::File;

pub async fn get_file_by_id(pool: &PgPool, file_id: &Uuid)
                            -> Result<Option<File>, sqlx::Error> {
    let file = sqlx::query_as!(
        File,
        "SELECT id, name, owner_id, parent_folder_id, file_type as \"file_type: _\" FROM files WHERE id = $1",
        file_id
    )
        .fetch_optional(pool)
        .await?;

    Ok(file)
}


pub async fn get_files_by_folder_id(pool: &PgPool, folder_id: &Uuid)
                                    -> Result<Vec<File>, sqlx::Error> {
    let files = sqlx::query_as!(
        File,
        "SELECT id, name, owner_id, parent_folder_id, file_type as \"file_type: _\" FROM files WHERE parent_folder_id = $1",
        folder_id
    )
        .fetch_all(pool)
        .await?;

    Ok(files)
}

pub async fn delete_file_by_id(pool: &PgPool, file_id: &Uuid)
                               -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM files WHERE id = $1", file_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn upload_file(file: File,pool: &PgPool)
    -> Result<File, sqlx::Error> {
    let file = sqlx::query_as!(
        File,
        "INSERT INTO files (id, name, owner_id, parent_folder_id, file_type) VALUES ($1,$2, $3, $4, $5) RETURNING id, name, owner_id,parent_folder_id, file_type as \"file_type:_ \"",
        file.id,
        file.name,
        file.owner_id,
        file.parent_folder_idm
        file.file_type
    )
        .fetch_one(pool)
        .await?;

    Ok(file)
}