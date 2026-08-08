use crate::features::ask_imam::errors::delete_question_error::DeleteQuestionError;
use crate::features::ask_imam::errors::upsert_answer_to_question_error::UpsertAnswerToQuestionError;
use crate::features::ask_imam::repositories::ImamQuestionsAdminRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::answer::Answer;
use masjid_app_api_library::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use masjid_app_api_library::features::ask_imam::models::school_of_thought::SchoolOfThought;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;

#[async_trait]
impl ImamQuestionsAdminRepository for InMemoryRepository {
    async fn get_all_imam_questions(&self) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        tracing::warn!("in-memory database not implemented for get_all_imam_questions");
        Err(GetQuestionsError::UnableToGetAnsweredQuestions)
    }

    async fn get_unanswered_imam_questions(
        &self,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        tracing::warn!("in-memory database not implemented for get_unanswered_imam_questions");
        Err(GetQuestionsError::UnableToGetAnsweredQuestions)
    }

    async fn get_unanswered_imam_questions_by_topic(
        &self,
        topic: &str,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        tracing::warn!(
            "in-memory database not implemented for get_unanswered_imam_questions_by_topic"
        );
        Err(GetQuestionsError::UnableToGetAnsweredQuestions)
    }

    async fn get_unanswered_imam_questions_by_school_of_thought(
        &self,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        tracing::warn!(
            "in-memory database not implemented for get_unanswered_imam_questions_by_school_of_thought"
        );
        Err(GetQuestionsError::UnableToGetAnsweredQuestions)
    }

    async fn get_unanswered_imam_questions_by_topic_and_school_of_thought(
        &self,
        topic: &str,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        tracing::warn!(
            "in-memory database not implemented for get_unanswered_imam_questions_by_topic_and_school_of_thought"
        );
        Err(GetQuestionsError::UnableToGetAnsweredQuestions)
    }

    async fn upsert_imam_answer_to_question(
        &self,
        question_id: &i32,
        answer: &Answer,
    ) -> Result<(), UpsertAnswerToQuestionError> {
        tracing::warn!("in-memory database not implemented for upsert_imam_answer_to_question");
        Err(UpsertAnswerToQuestionError::UnableToUpsertAnswerToQuestion)
    }

    async fn delete_imam_question_by_id(&self, id: &i32) -> Result<(), DeleteQuestionError> {
        tracing::warn!("in-memory database not implemented for delete_imam_question_by_id");
        Err(DeleteQuestionError::UnableToDeleteQuestion)
    }
}
