use crate::features::feedback::models::filters::get_feedback_filter::GetFeedbackFilter;
use crate::features::feedback::repositories::feedback_admin_repository::FeedbackAdminRepository;
use crate::features::feedback::services::errors::get_feedback_service_error::GetFeedbackServiceError;
use async_trait::async_trait;
use masjid_app_api_library::features::feedback::models::feedback_dto::FeedbackDTO;
use masjid_app_api_library::new_service;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait FeedbackRetrievalService: Send + Sync {
    async fn retrieve_feedback(
        &self,
        filter: GetFeedbackFilter,
    ) -> Result<Vec<FeedbackDTO>, GetFeedbackServiceError>;
}

new_service!(
    new_feedback_retrieval_service,
    FeedbackRetrievalService,
    FeedbackAdminRepository
);

#[async_trait]
impl FeedbackRetrievalService for CommonServiceImpl<dyn FeedbackAdminRepository> {
    async fn retrieve_feedback(
        &self,
        filter: GetFeedbackFilter,
    ) -> Result<Vec<FeedbackDTO>, GetFeedbackServiceError> {
        match self.in_memory_repository.get_feedback(&filter).await {
            Ok(feedback) => Ok(feedback),
            Err(_) => self
                .repository
                .get_feedback(&filter)
                .await
                .map_err(move |err| GetFeedbackServiceError::RepositoryError(err)),
        }
    }
}
