use async_trait::async_trait;

use crate::shared::services::file_management::{
    file_deletion::{error::FileDeletionError, service::FileDeletionService},
    file_upload::{error::FileUploadError, service::FileUploadService},
};

pub(super) struct FileManagementServiceImpl;

#[async_trait]
impl FileUploadService for FileManagementServiceImpl {
    async fn upload(&self, files: Vec<&[u8]>) -> Result<(), FileUploadError> {
        todo!()
    }
}

#[async_trait]
impl FileDeletionService for FileManagementServiceImpl {
    async fn delete(&self, dir: &str, ids: &[u64]) -> Result<(), FileDeletionError> {
        ids.into_iter()
            .map(|id| id.to_string())
            .for_each(move |id| {
                let result = std::fs::remove_dir(format!("{dir}/{image}"))
            });
    }
}
