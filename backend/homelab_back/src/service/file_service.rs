use sqlx::PgPool;
use uuid::Uuid;
use crate::data::upload_file_command::UploadFileCommand;
use crate::db::file_repository;
use crate::domain::file::{File, FileType};
use crate::domain::folder::Folder;
use crate::exception::data_error::{DataError};
use crate::service::folder_service;

pub async fn get_file_by_id (pool: &PgPool, file_id: &Uuid)
                             -> Result<Option<File>, sqlx::Error>{
    file_repository::get_file_by_id(pool, file_id).await
}

pub async fn get_files_by_folder(pool: &PgPool, folder_id: &Uuid)
-> Result<Vec<File>, sqlx::Error> {
    file_repository::get_files_by_folder_id(pool, folder_id).await
}

pub async fn delete_file (pool: &PgPool, file_id: &Uuid)
-> Result<(), sqlx::Error> {
    file_repository::delete_file_by_id(pool, file_id).await
}

pub async fn upload_file(pool: &PgPool, command: UploadFileCommand)
-> Result<File, DataError>{
    
    let folder: Folder = match folder_service::find_folder_by_id(pool, &command.destination_folder_id).await { 
        Ok(Some(folder)) => folder,
        Ok(None) => {
            return Err(DataError::EntityNotFoundException("Folder".to_string()));
        },
        Err(e) => {
            return Err(DataError::DatabaseError(e));
        }
    };

    // Reordering helps with the ownership problem. I first extracted the file type by borrowing value of the file name and then gave away the name when creating File bellow
    let file_type: FileType = File::get_file_type(&command.file.name);

    let file = File {
        id: Uuid::new_v4(),
        name: command.file.name,
        owner_id: command.owner_id,
        parent_folder_id: folder.id,
        file_type,
    }
}