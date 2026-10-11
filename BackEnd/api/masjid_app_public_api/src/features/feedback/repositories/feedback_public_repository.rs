use crate::features::feedback::errors::insert_feedback_repository_error::InsertFeedbackRepositoryError;
use async_trait::async_trait;
use masjid_app_api_library::features::feedback::models::feedback::Feedback;

#[async_trait]
pub trait FeedbackPublicRepository: Send + Sync {
    async fn insert_feedback(
        &self,
        feedback: &Feedback,
    ) -> Result<(), InsertFeedbackRepositoryError>;
}
