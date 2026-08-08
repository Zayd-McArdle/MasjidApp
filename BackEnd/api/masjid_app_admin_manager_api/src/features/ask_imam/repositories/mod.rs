use crate::features::ask_imam::errors::delete_question_error::DeleteQuestionError;
use crate::features::ask_imam::errors::upsert_answer_to_question_error::UpsertAnswerToQuestionError;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::answer::Answer;
use masjid_app_api_library::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use masjid_app_api_library::features::ask_imam::models::school_of_thought::SchoolOfThought;
use masjid_app_api_library::features::ask_imam::repositories::ImamQuestionsRepository;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use masjid_app_api_library::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use masjid_app_api_library::shared::data_access::repository_management::repository_mode::RepositoryMode;
use masjid_app_api_library::shared::data_access::repository_management::repository_type::RepositoryType;
use std::sync::Arc;

mod mysql_impl;
mod redis_impl;

#[async_trait]
pub trait ImamQuestionsAdminRepository: ImamQuestionsRepository {
    async fn get_all_imam_questions(&self) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_unanswered_imam_questions(
        &self,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_unanswered_imam_questions_by_topic(
        &self,
        topic: &str,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_unanswered_imam_questions_by_school_of_thought(
        &self,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_unanswered_imam_questions_by_topic_and_school_of_thought(
        &self,
        topic: &str,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn upsert_imam_answer_to_question(
        &self,
        question_id: &i32,
        answer: &Answer,
    ) -> Result<(), UpsertAnswerToQuestionError>;
    async fn delete_imam_question_by_id(&self, id: &i32) -> Result<(), DeleteQuestionError>;
}

pub async fn new_imam_questions_admin_repository(
    repository_mode: RepositoryMode,
) -> Arc<dyn ImamQuestionsAdminRepository> {
    new_repository!(repository_mode, RepositoryType::AskImam)
}
