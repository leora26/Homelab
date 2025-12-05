use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::data::file_folder::update_file_name_command::UpdateFileNameCommand;
use crate::data::file_folder::upload_file_command::UploadFileCommand;
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
    async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, DataError>;
    async fn get_all_deleted_files (&self) -> Result<Vec<File>, DataError>;
    async fn search_file (&self, search_query: String) -> Result<Vec<File>, DataError>;
    async fn upload(&self, command: UploadFileCommand) -> Result<File, DataError>;
    async fn update_file_name (&self, command: UpdateFileNameCommand, id: Uuid) -> Result<File, DataError>;
    async fn update_deleted_file (&self, id: Uuid) -> Result<File, DataError>;
    async fn delete_chosen_files (&self, file_ids: &[Uuid]) -> Result<(), DataError>;
    async fn delete(&self, file_id: Uuid) -> Result<(), DataError>;
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
    async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, DataError> {
        self.file_repo.get_by_id(file_id).await
    }

    async fn get_all_deleted_files(&self) -> Result<Vec<File>, DataError> {
        self.file_repo.get_all_deleted().await
    }

    async fn search_file(&self, search_query: String) -> Result<Vec<File>, DataError> {
        self.file_repo.search_by_name(format!("%{}%", search_query)).await
    }

    async fn upload(&self, command: UploadFileCommand) -> Result<File, DataError> {

        let folder_id = command.destination_folder_id;

        let folder: Folder = self.folder_repo.get_by_id(folder_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("Folder".to_string()))?;

        let user: User = self.user_repo.get_by_id(command.owner_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        let f = File::new(Uuid::new_v4(), command.file.name, user.id, folder.id, false);

        self.io_service.upload_file_to_disk(&command.file.data, &f).await.expect("TODO: panic message");

        self.file_repo.upload(f).await
    }

    async fn update_file_name(&self, command: UpdateFileNameCommand, file_id: Uuid) -> Result<File, DataError> {
        let mut file: File = self.file_repo.get_by_id(file_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.rename(command.new_name);

        self.file_repo.update(file).await
    }

    async fn update_deleted_file(&self, id: Uuid) -> Result<File, DataError> {
        let mut file: File = self.file_repo.get_by_id(id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.set_as_undeleted();

        self.file_repo.update(file).await
    }

    async fn delete_chosen_files(&self, file_ids: &[Uuid]) -> Result<(), DataError> {
        self.file_repo.delete_all(file_ids).await
    }

    async fn delete(&self, file_id: Uuid) -> Result<(), DataError> {

        let mut file = self.file_repo.get_by_id(file_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.set_as_deleted();

        self.file_repo.delete_by_id(file_id).await
    }
}