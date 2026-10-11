use crate::features::feedback::models::filters::delete_feedback_filter::DeleteFeedbackFilter;
use crate::features::feedback::models::filters::get_feedback_filter::GetFeedbackFilter;
use crate::features::feedback::repositories::errors::delete_feedback_by_id_repository_error::DeleteFeedbackByIdRepositoryError;
use crate::features::feedback::repositories::errors::delete_feedback_repository_error::DeleteFeedbackRepositoryError;
use crate::features::feedback::repositories::errors::get_feedback_repository_error::GetFeedbackRepositoryError;
use async_trait::async_trait;
use masjid_app_api_library::features::feedback::models::feedback_dto::FeedbackDTO;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use masjid_app_api_library::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use masjid_app_api_library::shared::data_access::repository_management::repository_mode::RepositoryMode;
use masjid_app_api_library::shared::data_access::repository_management::repository_type::RepositoryType;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait FeedbackAdminRepository: Send + Sync {
    async fn get_feedback(
        &self,
        filter: &GetFeedbackFilter,
    ) -> Result<Vec<FeedbackDTO>, GetFeedbackRepositoryError>;
    async fn delete_feedback_by_id(
        &self,
        id: &u64,
    ) -> Result<(), DeleteFeedbackByIdRepositoryError>;
    async fn delete_feedback(
        &self,
        filter: &DeleteFeedbackFilter,
    ) -> Result<(), DeleteFeedbackRepositoryError>;
}

pub async fn new_feedback_admin_repository(
    mode: RepositoryMode,
) -> Arc<dyn FeedbackAdminRepository> {
    new_repository!(mode, RepositoryType::Feedback)
}

#[async_trait]
impl FeedbackAdminRepository for InMemoryRepository {
    async fn get_feedback(
        &self,
        filter: &GetFeedbackFilter,
    ) -> Result<Vec<FeedbackDTO>, GetFeedbackRepositoryError> {
        Err(GetFeedbackRepositoryError::UnableToRetrieveFeedback)
    }

    async fn delete_feedback_by_id(
        &self,
        id: &u64,
    ) -> Result<(), DeleteFeedbackByIdRepositoryError> {
        Err(DeleteFeedbackByIdRepositoryError::UnableToDeleteFeedbackId)
    }

    async fn delete_feedback(
        &self,
        filter: &DeleteFeedbackFilter,
    ) -> Result<(), DeleteFeedbackRepositoryError> {
        Err(DeleteFeedbackRepositoryError::UnableToDeleteFeedback)
    }
}

#[async_trait]
impl FeedbackAdminRepository for MySqlRepository {
    async fn get_feedback(
        &self,
        filter: &GetFeedbackFilter,
    ) -> Result<Vec<FeedbackDTO>, GetFeedbackRepositoryError> {
        todo!()
    }
    async fn delete_feedback_by_id(
        &self,
        id: &u64,
    ) -> Result<(), DeleteFeedbackByIdRepositoryError> {
        todo!()
    }
    async fn delete_feedback(
        &self,
        filter: &DeleteFeedbackFilter,
    ) -> Result<(), DeleteFeedbackRepositoryError> {
        todo!()
    }
}
