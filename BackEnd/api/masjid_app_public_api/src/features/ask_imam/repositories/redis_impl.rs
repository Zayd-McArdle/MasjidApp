use crate::features::ask_imam::errors::insert_imam_question_error::InsertImamQuestionError;
use crate::features::ask_imam::repositories::ImamQuestionsPublicRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::models::imam_question::ImamQuestion;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;

#[async_trait]
impl ImamQuestionsPublicRepository for InMemoryRepository {
    async fn insert_question_for_imam(
        &self,
        questions: &ImamQuestion,
    ) -> Result<(), InsertImamQuestionError> {
        tracing::warn!("in-memory database not implemented for insert_question_for_imam");
        Err(InsertImamQuestionError::UnableToInsertQuestion)
    }
}
