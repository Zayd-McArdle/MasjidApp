use std::sync::Arc;

use crate::shared::services::file_management::{
    file_deletion::error::FileDeletionError, file_management_service::FileManagementServiceImpl,
};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait FileDeletionService: Send + Sync {
    async fn delete(&self, dir: &str, ids: &[u64]) -> Result<(), FileDeletionError>;
}

pub fn new_file_deletion_service() -> Arc<dyn FileDeletionService> {
    Arc::new(FileManagementServiceImpl)
}
