use async_trait::async_trait;
use masjid_app_api_library::features::feedback::models::feedback_dto::FeedbackDTO;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use crate::features::feedback::models::feedback_deletion_mode::FeedbackDeletionMode;
use crate::features::feedback::repositories::errors::delete_feedback_error::DeleteFeedbackError;
use crate::features::feedback::repositories::errors::get_feedback_error::GetFeedbackError;
use crate::features::feedback::models::filters::get_feedback_filter::GetFeedbackFilter;

#[async_trait]
pub trait FeedbackAdminRepository: Send + Sync {
    async fn get_feedback(&self, filter: &GetFeedbackFilter) -> Result<Vec<FeedbackDTO>, GetFeedbackError>;
    async fn delete_feedback(&self, mode: &FeedbackDeletionMode) -> Result<(), DeleteFeedbackError>;
}

#[async_trait]
impl FeedbackAdminRepository for MySqlRepository {
    async fn get_feedback(&self, filter: &GetFeedbackFilter) -> Result<Vec<FeedbackDTO>, GetFeedbackError> {
        todo!()
    }
    async fn delete_feedback(&self, mode: &FeedbackDeletionMode) -> Result<(), DeleteFeedbackError> {
        todo!()
    }
}