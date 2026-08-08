use crate::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use crate::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use crate::features::ask_imam::models::school_of_thought::SchoolOfThought;
use crate::features::ask_imam::repositories::ImamQuestionsRepository;
use crate::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use async_trait::async_trait;

#[async_trait]
impl ImamQuestionsRepository for InMemoryRepository {
    async fn get_answered_questions(&self) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        tracing::warn!("in-memory database not implemented for get_answered_questions");
        Err(GetQuestionsError::UnableToGetAnsweredQuestions)
    }

    async fn get_answered_questions_by_topic(
        &self,
        topic: &str,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        tracing::warn!("in-memory database not implemented for get_answered_questions_by_topic");
        Err(GetQuestionsError::UnableToGetAnsweredQuestions)
    }

    async fn get_answered_questions_by_school_of_thought(
        &self,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        tracing::warn!(
            "in-memory database not implemented for get_answered_questions_by_school_of_thought"
        );
        Err(GetQuestionsError::UnableToGetAnsweredQuestions)
    }

    async fn get_answered_questions_by_topic_and_school_of_thought(
        &self,
        topic: &str,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        tracing::warn!(
            "in-memory database not implemented for get_answered_questions_by_topic_and_school_of_thought"
        );
        Err(GetQuestionsError::UnableToGetAnsweredQuestions)
    }
}
