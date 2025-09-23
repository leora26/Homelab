use std::sync::Arc;
use async_recursion::async_recursion;
use async_trait::async_trait;
use uuid::Uuid;
use crate::db::folder_repository::FolderRepository;
use crate::domain::folder::Folder;
use crate::exception::data_error::DataError;

#[async_trait]
pub trait FolderService: Send + Sync {
    async fn get_root(&self, user_id: &Uuid) -> Result<Option<Folder>, DataError>;
    async fn get_by_id(&self, folder_id: &Uuid) -> Result<Option<Folder>, DataError>;
    async fn get_children_by_id(&self, folder_id: &Uuid) -> Result<Vec<Folder>, DataError>;
    async fn delete(&self, folder_id: &Uuid) -> Result<(), DataError>;
    async fn get_folder_path(&self, folder_id: &Uuid) -> Result<String, DataError>;
}

pub struct FolderServiceImpl {
    folder_repo: Arc<dyn FolderRepository>,
}

impl FolderServiceImpl {
    pub fn new(folder_repo: Arc<dyn FolderRepository>) -> Self {
        Self { folder_repo }
    }

    #[async_recursion]
    async fn get_parent_folder_name(&self, f_id: &Uuid) -> Result<String, DataError> {
        let f = self.folder_repo.get_by_id(f_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("Folder".to_string()))?;

        if let Some(parent_id) = f.parent_folder_id {
            let parent_path = self.get_parent_folder_name(&parent_id).await?;
            Ok(format!("{}/{}", parent_path, f.name))
        } else {
            Ok(f.name)
        }
    }
}

#[async_trait]
impl FolderService for FolderServiceImpl {
    async fn get_root(&self, user_id: &Uuid) -> Result<Option<Folder>, DataError> {
        self.folder_repo.get_root(user_id).await
    }

    async fn get_by_id(&self, folder_id: &Uuid) -> Result<Option<Folder>, DataError> {
        self.folder_repo.get_by_id(folder_id).await
    }

    async fn get_children_by_id(&self, folder_id: &Uuid) -> Result<Vec<Folder>, DataError> {
        self.folder_repo.get_children_by_id(folder_id).await
    }

    async fn delete(&self, folder_id: &Uuid) -> Result<(), DataError> {
        self.folder_repo.delete_by_id(folder_id).await
    }

    async fn get_folder_path(&self, folder_id: &Uuid) -> Result<String, DataError> {
        let path = self.get_parent_folder_name(folder_id).await?;
        Ok(path)
    }
}