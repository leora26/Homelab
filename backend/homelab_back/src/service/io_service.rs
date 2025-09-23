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
}

impl IOServiceImpl {
    pub fn new(folder_service: Arc<dyn FolderService>) -> Self {
        Self { folder_service }
    }
}

impl IOService for IOServiceImpl {
    async fn upload_file_to_disk(&self, file_content: &Vec<u8>, f: &File) -> Result<()> {
       let path = match self.folder_service.get_folder_path(&f.parent_folder_id).await {
           Ok(p) => p,
           Err(e) => {
               return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Path not found: {}", e)));
           }
       };

        let full_path = format!("{}/{}", path, f.name);

        fs::write(&full_path, file_content).await?;

        Ok(())
    }
}