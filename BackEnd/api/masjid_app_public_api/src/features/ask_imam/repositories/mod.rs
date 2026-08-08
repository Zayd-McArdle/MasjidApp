mod mysql_impl;
mod redis_impl;

use crate::features::ask_imam::errors::insert_imam_question_error::InsertImamQuestionError;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::models::imam_question::ImamQuestion;
use masjid_app_api_library::features::ask_imam::repositories::ImamQuestionsRepository;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use masjid_app_api_library::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use masjid_app_api_library::shared::data_access::repository_management::repository_mode::RepositoryMode;
use masjid_app_api_library::shared::data_access::repository_management::repository_type::RepositoryType;
use std::sync::Arc;

#[async_trait]
pub trait ImamQuestionsPublicRepository: ImamQuestionsRepository {
    async fn insert_question_for_imam(
        &self,
        questions: &ImamQuestion,
    ) -> Result<(), InsertImamQuestionError>;
}

pub async fn new_imam_questions_public_repository(
    repository_mode: RepositoryMode,
) -> Arc<dyn ImamQuestionsPublicRepository> {
    new_repository!(repository_mode, RepositoryType::AskImam)
}
