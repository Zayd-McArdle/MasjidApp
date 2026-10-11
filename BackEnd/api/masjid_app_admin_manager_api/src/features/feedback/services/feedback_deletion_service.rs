use crate::features::feedback::models::feedback_deletion_mode::FeedbackDeletionMode;
use crate::features::feedback::repositories::feedback_admin_repository::FeedbackAdminRepository;
use crate::features::feedback::services::errors::delete_feedback_service_error::DeleteFeedbackServiceError;
use async_trait::async_trait;
use masjid_app_api_library::new_service;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait FeedbackDeletionService: Send + Sync {
    async fn delete_feedback(
        &self,
        mode: FeedbackDeletionMode,
    ) -> Result<(), DeleteFeedbackServiceError>;
}

new_service!(
    new_feedback_deletion_service,
    FeedbackDeletionService,
    FeedbackAdminRepository
);

#[async_trait]
impl FeedbackDeletionService for CommonServiceImpl<dyn FeedbackAdminRepository> {
    async fn delete_feedback(
        &self,
        mode: FeedbackDeletionMode,
    ) -> Result<(), DeleteFeedbackServiceError> {
        match mode {
            FeedbackDeletionMode::Single(id) => {
                if let Err(in_memory_repository_error) =
                    self.in_memory_repository.delete_feedback_by_id(&id).await
                {
                    tracing::warn!(
                        ?in_memory_repository_error,
                        ?id,
                        "Unable to delete feedback by id from in-memory database"
                    );
                }
                self.repository
                    .delete_feedback_by_id(&id)
                    .await
                    .map_err(move |err| {
                        DeleteFeedbackServiceError::RepositorySingleDeletionError(err)
                    })
            }
            FeedbackDeletionMode::Multiple(filter) => {
                if let Err(in_memory_repository_error) =
                    self.in_memory_repository.delete_feedback(&filter).await
                {
                    tracing::warn!(
                        ?in_memory_repository_error,
                        ?filter,
                        "unable to delete feedback from in-memory database using the following filters"
                    );
                }
                self.repository
                    .delete_feedback(&filter)
                    .await
                    .map_err(move |err| {
                        DeleteFeedbackServiceError::RepositoryMultiDeletionError(err)
                    })
            }
        }
    }
}
