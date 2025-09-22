use sqlx::PgPool;
use uuid::Uuid;
use crate::data::upload_file_command::UploadFileCommand;
use crate::db::file_repository;
use crate::domain::file::{File};
use crate::domain::folder::Folder;
use crate::domain::user::User;
use crate::exception::data_error::{DataError};
use crate::service::{folder_service, io_service, user_service};

pub async fn get_file_by_id(pool: &PgPool, file_id: &Uuid)
                            -> Result<Option<File>, sqlx::Error> {
    file_repository::get_file_by_id(pool, file_id).await
}

pub async fn get_files_by_folder(pool: &PgPool, folder_id: &Uuid)
                                 -> Result<Vec<File>, sqlx::Error> {
    file_repository::get_files_by_folder_id(pool, folder_id).await
}

pub async fn delete_file(pool: &PgPool, file_id: &Uuid)
                         -> Result<(), sqlx::Error> {
    file_repository::delete_file_by_id(pool, file_id).await
}

pub async fn upload_file(pool: &PgPool, command: UploadFileCommand)
                         -> Result<File, DataError> {
    let folder: Folder = match folder_service::find_folder_by_id(pool, &command.destination_folder_id).await {
        Ok(Some(folder)) => folder,
        Ok(None) => {
            return Err(DataError::EntityNotFoundException("Folder".to_string()));
        }
        Err(e) => {
            return Err(DataError::DatabaseError(e));
        }
    };

    let user: User = match user_service::get_user_by_id(pool, &command.owner_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(DataError::EntityNotFoundException("User".to_string()));
        }
        Err(e) => {
            return Err(DataError::DatabaseError(e))
        }
    };

    let f = File::new(Uuid::new_v4(), command.file.name, user.id, folder.id);

    let _ = io_service::upload_file_to_disk(&command.file.data, &f, pool).await;

    match file_repository::upload_file(f, pool).await {
        Ok(file) => Ok(file),
        Err(e) => Err(DataError::DatabaseError(e))
    }
}