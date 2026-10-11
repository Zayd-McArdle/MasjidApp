use std::sync::Arc;

use crate::shared::services::file_management::{
    file_management_service::FileManagementServiceImpl, file_upload::error::FileUploadError,
};
use async_trait::async_trait;
#[async_trait]
pub trait FileUploadService: Send + Sync {
    async fn upload(&self, files: Vec<&[u8]>) -> Result<(), FileUploadError>;
}

pub fn new_file_upload_service() -> Arc<dyn FileUploadService> {
    Arc::new(FileManagementServiceImpl)
}
