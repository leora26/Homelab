use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::data::upload_file_command::UploadFileCommand;
use crate::db::file_repository::FileRepository;
use crate::db::folder_repository::FolderRepository;
use crate::db::user_repository::UserRepository;
use crate::domain::file::{File};
use crate::domain::folder::Folder;
use crate::domain::user::User;
use crate::exception::data_error::{DataError};
use crate::service::io_service::IOService;

#[async_trait]
pub trait FileService: Send + Sync {
    async fn get_by_id(&self, file_id: &Uuid) -> Result<Option<File>, DataError>;
    async fn delete(&self, file_id: &Uuid) -> Result<(), DataError>;
    async fn upload(&self, command: UploadFileCommand) -> Result<File, DataError>;
}

pub struct FileServiceImpl {
    file_repo: Arc<dyn FileRepository>,
    folder_repo: Arc<dyn FolderRepository>,
    user_repo: Arc<dyn UserRepository>,
    io_service: Arc<dyn IOService>,
}

impl FileServiceImpl {
    pub fn new(
        file_repo: Arc<dyn FileRepository>,
        folder_repo: Arc<dyn FolderRepository>,
        user_repo: Arc<dyn UserRepository>,
        io_service: Arc<dyn IOService>,
    ) -> Self {
        Self {
            file_repo,
            folder_repo,
            user_repo,
            io_service,
        }
    }
}

#[async_trait]
impl FileService for FileServiceImpl {
    async fn get_by_id(&self, file_id: &Uuid) -> Result<Option<File>, DataError> {
        self.file_repo.get_by_id(file_id).await
    }

    async fn delete(&self, file_id: &Uuid) -> Result<(), DataError> {
        self.file_repo.delete_by_id(file_id).await
    }

    async fn upload(&self, command: UploadFileCommand) -> Result<File, DataError> {
        let folder: Folder = self.folder_repo.get_by_id(&command.destination_folder_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("Folder".to_string()))?;

        let user: User = self.user_repo.get_by_id(&command.owner_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;


        let f = File::new(Uuid::new_v4(), command.file.name, user.id, folder.id);

        let _ = self.io_service.upload_file_to_disk(&command.file.data, &f).await;

        self.file_repo.upload(f).await
    }
}