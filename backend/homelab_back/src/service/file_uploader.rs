use std::sync::Arc;
use actix_web::web::Payload;
use futures::StreamExt;
use tokio::io::{AsyncWriteExt, BufWriter};
use uuid::Uuid;
use crate::constants::MB;
use crate::db::file_repository::FileRepository;
use crate::domain::file::UploadStatus;
use crate::exception::data_error::DataError;


pub struct FileUploader {
    file_repo: Arc<dyn FileRepository>,
}

// TODO: Marked for removal since there is a new implementation for gRPC
impl FileUploader {
    pub fn new(file_repo: Arc<dyn FileRepository>) -> Self {
        Self { file_repo }
    }

    pub async fn upload_content(&self, file_id: Uuid, mut payload: Payload) -> Result<(), DataError> {

        let mut f = self.file_repo.get_by_id(file_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        if f.upload_status != UploadStatus::Pending {
            return Err(DataError::ValidationError("File is not pending".to_string()));
        }

        let file_path = f.build_file_path();

        let file_handle = tokio::fs::File::create(&file_path).await
            .map_err(|e| DataError::IOError(e.to_string()))?;

        let mut writer = BufWriter::with_capacity(MB as usize, file_handle);

        let mut total_bytes = 0i64;

        while let Some(chunk) = payload.next().await {
            let data = chunk.map_err(|e| DataError::UploadInterrupter(e.to_string()))?;

            total_bytes += data.len() as i64;

            if let Err(e) = writer.write_all(&data).await {
                let _ = tokio::fs::remove_file(&file_path).await;
                return Err(DataError::IOError(e.to_string()));
            }
        }

        if let Err(e) = writer.flush().await {
            let _ = tokio::fs::remove_file(&file_path).await;
            return Err(DataError::IOError(e.to_string()));
        }

        if !f.validate_size(total_bytes) {
            f.update_status(UploadStatus::Failed);
            self.file_repo.update(f).await?;

            let _ = tokio::fs::remove_file(&file_path).await;

            return Err(DataError::NotMatchingByteSizeError);
        }

        f.update_status(UploadStatus::Completed);
        self.file_repo.update(f).await?;

        Ok(())
    }
}