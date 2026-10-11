use crate::features::feedback::errors::send_feedback_error::SendFeedbackError;
use crate::features::feedback::repositories::feedback_public_repository::FeedbackPublicRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::feedback::models::feedback::Feedback;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use mockall::automock;

#[automock]
#[async_trait]
pub trait FeedbackPublishingService: Send + Sync {
    async fn send_feedback(&self, feedback: Feedback) -> Result<(), SendFeedbackError>;
}

#[async_trait]
impl FeedbackPublishingService for CommonServiceImpl<dyn FeedbackPublicRepository> {
    async fn send_feedback(&self, feedback: Feedback) -> Result<(), SendFeedbackError> {
        self.repository
            .insert_feedback(&feedback)
            .await
            .map_err(SendFeedbackError::from)?;
        if let Err(in_memory_repository_err) =
            self.in_memory_repository.insert_feedback(&feedback).await
        {
            tracing::warn!(error = ?in_memory_repository_err, "unable to insert feedback into in-memory repository into database");
        }
        Ok(())
    }
}
