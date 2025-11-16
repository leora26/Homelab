use std::sync::Arc;
use async_recursion::async_recursion;
use async_trait::async_trait;
use uuid::Uuid;
use crate::data::update_folder_name_command::UpdateFolderNameCommand;
use crate::db::folder_repository::FolderRepository;
use crate::domain::file::File;
use crate::domain::folder::Folder;
use crate::exception::data_error::DataError;

#[async_trait]
pub trait FolderService: Send + Sync {
    async fn get_root (&self, user_id: &Uuid) -> Result<Option<Folder>, DataError>;
    async fn get_by_id (&self, folder_id: &Uuid) -> Result<Option<Folder>, DataError>;
    async fn get_children_by_id (&self, folder_id: &Uuid) -> Result<Vec<Folder>, DataError>;
    async fn delete (&self, folder_id: &Uuid) -> Result<(), DataError>;
    async fn get_folder_path (&self, folder_id: &Uuid) -> Result<String, DataError>;
    async fn get_by_folder (&self, folder_id: &Uuid) -> Result<Vec<File>, DataError>;
    async fn update_folder_name (&self, command: UpdateFolderNameCommand, folder_id: Uuid) -> Result<Folder, DataError>;
    async fn search_folder (&self, search_query: String) -> Result<Vec<Folder>, DataError>;
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

    async fn get_by_folder(&self, folder_id: &Uuid) -> Result<Vec<File>, DataError> {
        self.folder_repo.get_by_folder_id(folder_id).await
    }

    async fn update_folder_name(&self, command: UpdateFolderNameCommand, folder_id: Uuid) -> Result<Folder, DataError> {
        let mut folder: Folder = self.folder_repo.get_by_id(&folder_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        folder.rename(command.new_name);

        self.folder_repo.update_folder(folder).await
    }

    async fn search_folder(&self, search_query: String) -> Result<Vec<Folder>, DataError> {
        self.folder_repo.search_by_name(format!("%{}%", search_query)).await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use time::OffsetDateTime;
    use super::*;

    pub struct MockFolderRepo {
        folders: HashMap<Uuid, Folder>,
    }

    impl MockFolderRepo {
        fn new() -> Self {
            Self {
                folders: HashMap::new()
            }
        }

        fn add_folder(&mut self, folder: Folder) {
            self.folders.insert(folder.id, folder);
        }
    }

    #[async_trait]
    impl FolderRepository for MockFolderRepo {
        async fn get_by_id(&self, folder_id: &Uuid) -> Result<Option<Folder>, DataError> {
            Ok(self.folders.get(folder_id).map(|f| Folder {
                id: f.id,
                parent_folder_id: f.parent_folder_id,
                name: f.name.clone(),
                owner_id: f.owner_id,
                created_at: f.created_at.clone(),
            }))
        }
        async fn get_root(&self, user_id: &Uuid) -> Result<Option<Folder>, DataError> { unimplemented!() }
        async fn get_children_by_id(&self, folder_id: &Uuid) -> Result<Vec<Folder>, DataError> { unimplemented!() }
        async fn delete_by_id(&self, folder_id: &Uuid) -> Result<(), DataError> { unimplemented!() }
        async fn create(&self, folder: &Folder) -> Result<Folder, DataError> { unimplemented!() }

        async fn get_by_folder_id(&self, folder_id: &Uuid) -> Result<Vec<File>, DataError> { unimplemented!() }

        async fn update_folder(&self, folder: Folder) -> Result<Folder, DataError> {
            unimplemented!()
        }

        async fn search_by_name(&self, search_query: String) -> Result<Vec<Folder>, DataError> {
            unimplemented!()
        }
    }

    fn create_test_folder(id: Uuid, name: &str, parent_id: Option<Uuid>) -> Folder {
        Folder {
            id,
            parent_folder_id: parent_id,
            name: name.to_string(),
            owner_id: Uuid::new_v4(),
            created_at: OffsetDateTime::now_utc(),
        }
    }

    #[tokio::test]
    async fn should_get_correct_path_to_destination_folder () {
        let root_id = Uuid::new_v4();
        let child_id = Uuid::new_v4();
        let grandchild_id = Uuid::new_v4();

        let mut mock_repo = MockFolderRepo::new();
        mock_repo.add_folder(create_test_folder(root_id, "root", None));
        mock_repo.add_folder(create_test_folder(child_id, "child", Some(root_id)));
        mock_repo.add_folder(create_test_folder(grandchild_id, "grandchild", Some(child_id)));


        let folder_service = FolderServiceImpl::new(Arc::new(mock_repo));

        let result = folder_service.get_folder_path(&grandchild_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "root/child/grandchild");
    }

    #[tokio::test]
    async fn should_throw_error_because_of_invalid_folder() {
        let root_id = Uuid::new_v4();
        let child_id = Uuid::new_v4();
        let grandchild_id = Uuid::new_v4();

        let mut mock_repo = MockFolderRepo::new();
        mock_repo.add_folder(create_test_folder(root_id, "root", None));
        mock_repo.add_folder(create_test_folder(child_id, "child", Some(root_id)));
        mock_repo.add_folder(create_test_folder(grandchild_id, "grandchild", Some(child_id)));


        let folder_service = FolderServiceImpl::new(Arc::new(mock_repo));

        let result = folder_service.get_folder_path(&Uuid::new_v4()).await;

        assert!(result.is_err());
    }

}