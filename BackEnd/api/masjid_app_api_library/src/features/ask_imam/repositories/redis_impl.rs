use crate::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use crate::features::ask_imam::models::get_imam_questions_filter::GetImamQuestionsFilter;
use crate::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use crate::features::ask_imam::repositories::ImamQuestionsRepository;
use crate::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use async_trait::async_trait;

#[async_trait]
impl ImamQuestionsRepository for InMemoryRepository {
    async fn get_questions(
        &self,
        filter: &GetImamQuestionsFilter,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        tracing::warn!("in-memory database not implemented for get_answered_questions");
        Err(GetQuestionsError::UnableToGetAnsweredQuestions)
    }
}
