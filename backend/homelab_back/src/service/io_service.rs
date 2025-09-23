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
        let path = match self.folder_service.get_folder_path(&f.parent_folder_id).await {
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

