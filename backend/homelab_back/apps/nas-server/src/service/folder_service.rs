use crate::data::create_folder_command::CreateFolderCommand;
use crate::data::move_folder_command::MoveFolderCommand;
use crate::data::update_folder_name_command::UpdateFolderNameCommand;
use crate::db::folder_repository::FolderRepository;
use crate::helpers::data_error::DataError;
use async_recursion::async_recursion;
use async_trait::async_trait;
use homelab_core::file::{File, FileType};
use homelab_core::folder::Folder;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait FolderService: Send + Sync {
    async fn get_root(&self, user_id: Uuid) -> Result<Option<Folder>, DataError>;
    async fn get_by_id(&self, folder_id: Uuid) -> Result<Option<Folder>, DataError>;
    async fn get_children_by_id(&self, folder_id: Uuid) -> Result<Vec<Folder>, DataError>;
    async fn search_folder(&self, search_query: String) -> Result<Vec<Folder>, DataError>;
    async fn filter_files_by_folder(
        &self,
        file_types: &[FileType],
        folder_id: Uuid,
    ) -> Result<Vec<File>, DataError>;
    async fn get_folder_path(&self, folder_id: Uuid) -> Result<String, DataError>;
    async fn get_by_folder(&self, folder_id: Uuid) -> Result<Vec<File>, DataError>;
    async fn update_folder_name(
        &self,
        command: UpdateFolderNameCommand,
        folder_id: Uuid,
    ) -> Result<Folder, DataError>;
    async fn delete_chosen_folders(&self, folder_ids: &[Uuid]) -> Result<(), DataError>;
    async fn delete(&self, folder_id: Uuid) -> Result<(), DataError>;
    async fn create(&self, command: CreateFolderCommand) -> Result<Folder, DataError>;
    async fn move_folder(&self, command: MoveFolderCommand) -> Result<Folder, DataError>;
}

pub struct FolderServiceImpl {
    folder_repo: Arc<dyn FolderRepository>,
}

impl FolderServiceImpl {
    pub fn new(folder_repo: Arc<dyn FolderRepository>) -> Self {
        Self { folder_repo }
    }

    #[async_recursion]
    async fn get_parent_folder_name(&self, f_id: Uuid) -> Result<String, DataError> {
        let f = self
            .folder_repo
            .get_by_id(f_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("Folder".to_string()))?;

        if let Some(parent_id) = f.parent_folder_id {
            let parent_path = self.get_parent_folder_name(parent_id).await?;
            Ok(format!("{}/{}", parent_path, f.name))
        } else {
            Ok(f.name)
        }
    }
}

#[async_trait]
impl FolderService for FolderServiceImpl {
    async fn get_root(&self, user_id: Uuid) -> Result<Option<Folder>, DataError> {
        self.folder_repo.get_root(user_id).await
    }

    async fn get_by_id(&self, folder_id: Uuid) -> Result<Option<Folder>, DataError> {
        self.folder_repo.get_by_id(folder_id).await
    }

    async fn get_children_by_id(&self, folder_id: Uuid) -> Result<Vec<Folder>, DataError> {
        self.folder_repo.get_children_by_id(folder_id).await
    }

    async fn search_folder(&self, search_query: String) -> Result<Vec<Folder>, DataError> {
        self.folder_repo
            .search_by_name(format!("%{}%", search_query))
            .await
    }

    async fn filter_files_by_folder(
        &self,
        file_types: &[FileType],
        folder_id: Uuid,
    ) -> Result<Vec<File>, DataError> {
        self.folder_repo
            .filter_files_in_folder(file_types, folder_id)
            .await
    }

    async fn get_folder_path(&self, folder_id: Uuid) -> Result<String, DataError> {
        let path = self.get_parent_folder_name(folder_id).await?;
        Ok(path)
    }

    async fn get_by_folder(&self, folder_id: Uuid) -> Result<Vec<File>, DataError> {
        self.folder_repo.get_by_folder_id(folder_id).await
    }

    async fn update_folder_name(
        &self,
        command: UpdateFolderNameCommand,
        folder_id: Uuid,
    ) -> Result<Folder, DataError> {
        let mut folder: Folder = self
            .folder_repo
            .get_by_id(folder_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        folder.rename(command.new_name);

        self.folder_repo.update_folder(folder).await
    }

    async fn delete_chosen_folders(&self, folder_ids: &[Uuid]) -> Result<(), DataError> {
        self.folder_repo.delete_all(folder_ids).await
    }

    async fn delete(&self, folder_id: Uuid) -> Result<(), DataError> {
        self.folder_repo.delete_by_id(folder_id).await
    }

    async fn create(&self, command: CreateFolderCommand) -> Result<Folder, DataError> {
        let f = Folder::new(
            Uuid::new_v4(),
            Some(command.parent_folder_id),
            command.name,
            command.owner_id,
        );

        self.folder_repo.create(f).await
    }

    async fn move_folder(&self, command: MoveFolderCommand) -> Result<Folder, DataError> {
        let mut folder = self
            .folder_repo
            .get_by_id(command.folder_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("Folder".to_string()))?;

        folder.update_parent_folder(command.target_folder);

        Ok(self.folder_repo.update_folder(folder).await?)
    }
}
