use std::env;
use tokio::fs;
use crate::domain::file::File;
use std::io::Result;
use std::sync::Arc;
use async_trait::async_trait;
use crate::service::folder_service::FolderService;


#[async_trait]
pub trait IOService: Send + Sync {
    async fn upload_file_to_disk(&self, file_content: &Vec<u8>, f: &File) -> Result<()>;
}

pub struct IOServiceImpl {
    folder_service: Arc<dyn FolderService>,
    root_folder_path: String,
}

impl IOServiceImpl {
    pub fn new(folder_service: Arc<dyn FolderService>) -> Self {
        let root_folder_path = env::var("ROOT_FOLDER_PATH")
            .expect("ROOT_FOLDER_PATH must be set in .env file to be able to use it in the IOService");

        Self { folder_service, root_folder_path }
    }
}

#[async_trait]
impl IOService for IOServiceImpl {
    async fn upload_file_to_disk(&self, file_content: &Vec<u8>, f: &File) -> Result<()> {
        let path = match self.folder_service.get_folder_path(f.parent_folder_id).await {
            Ok(p) => p,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Path not found: {}", e)));
            }
        };

        let full_path = format!("{}/{}/{}", self.root_folder_path, path, f.name);

        fs::write(&full_path, file_content).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use uuid::Uuid;
    use crate::domain::folder::Folder;
    use crate::exception::data_error::DataError;
    use super::*;

    pub struct MockFolder;


    struct MockFolderService {
        return_error: bool,
    }

    #[async_trait]
    impl FolderService for MockFolderService {
        async fn get_folder_path(&self, folder_id: &Uuid) -> std::result::Result<String, DataError> {
            if self.return_error {
                Err(DataError::EntityNotFoundException("Mock folder not found".to_string()))
            } else {
                Ok("".to_string())
            }
        }
        async fn get_root(&self, user_id: &Uuid) -> std::result::Result<Option<Folder>, DataError> { unimplemented!() }

        async fn get_by_id(&self, folder_id: &Uuid) -> std::result::Result<Option<Folder>, DataError> { unimplemented!() }

        async fn get_children_by_id(&self, folder_id: &Uuid) -> std::result::Result<Vec<Folder>, DataError> { unimplemented!() }

        async fn delete(&self, folder_id: &Uuid) -> std::result::Result<(), DataError> { unimplemented!() }

        async fn get_by_folder(&self, folder_id: &Uuid) -> std::result::Result<Vec<File>, DataError> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn should_upload_file_and_return_success() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let root_path_str = temp_dir.path().to_str().unwrap();

        unsafe { env::set_var("ROOT_FOLDER_PATH", root_path_str); }

        let mock_folder_service = Arc::new(MockFolderService { return_error: false });
        let io_service = IOServiceImpl::new(mock_folder_service);

        let test_file = File::new(
            Uuid::new_v4(),
            "test_file_saving.txt".to_string(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        );

        let file_content = b"This is a test file to find out if i made IO function correct :)".to_vec();

        let result = io_service.upload_file_to_disk(&file_content, &test_file).await;

        assert!(result.is_ok());

        let expected_path = temp_dir.path().join("test_file_saving.txt");
        assert!(expected_path.exists(), "The test file should exist on disk.");

        let saved_content = fs::read(&expected_path).await.expect("Failed to read saved file");
        assert_eq!(saved_content, file_content, "The content of the saved file should match the original content.");

        env::remove_var("ROOT_FOLDER_PATH");
    }

    #[tokio::test]
    async fn should_not_find_folder_and_give_error() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let root_path_str = temp_dir.path().to_str().unwrap();

        unsafe { env::set_var("ROOT_FOLDER_PATH", root_path_str); }

        let mock_folder_service = Arc::new(MockFolderService { return_error: true });
        let io_service = IOServiceImpl::new(mock_folder_service);

        let test_file = File::new(
            Uuid::new_v4(),
            "test_file_saving.txt".to_string(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        );

        let file_content = b"This is a test file to find out if i made IO function correct :)".to_vec();

        let result = io_service.upload_file_to_disk(&file_content, &test_file).await;
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }
}

