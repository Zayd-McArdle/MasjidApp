use crate::features::ask_imam::errors::{DeleteQuestionError, UpsertAnswerToQuestionError};
use crate::features::ask_imam::models::QuestionStatus;
use crate::features::ask_imam::repositories::ImamQuestionsAdminRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::errors::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::{
    Answer, ImamQuestionDTO, SchoolOfThought,
};
use masjid_app_api_library::features::ask_imam::services::AskImamServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait AskImamAdminService: Send + Sync {
    async fn get_questions(
        &self,
        answered: Option<QuestionStatus>,
        topic: Option<String>,
        school_of_thought: Option<SchoolOfThought>,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn provide_answer_to_question(
        &self,
        question_id: i32,
        answer: Answer,
    ) -> Result<(), UpsertAnswerToQuestionError>;
    async fn delete_question(&self, question_id: i32) -> Result<(), DeleteQuestionError>;
}

pub fn new_ask_imam_admin_service(
    repository: Arc<dyn ImamQuestionsAdminRepository>,
    in_memory_repository: Arc<dyn ImamQuestionsAdminRepository>,
) -> Arc<dyn AskImamAdminService> {
    Arc::new(AskImamServiceImpl {
        repository,
        in_memory_repository,
    })
}

#[async_trait]
impl AskImamAdminService for AskImamServiceImpl<dyn ImamQuestionsAdminRepository> {
    async fn get_questions(
        &self,
        answered: Option<QuestionStatus>,
        topic: Option<String>,
        school_of_thought: Option<SchoolOfThought>,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        match (answered, topic, school_of_thought) {
            (None, _, _) => self
                .in_memory_repository
                .get_all_imam_questions()
                .await
                .or(self.repository.get_all_imam_questions().await),
            (Some(QuestionStatus::Unanswered), None, None) => self
                .in_memory_repository
                .get_unanswered_imam_questions()
                .await
                .or(self.repository.get_unanswered_imam_questions().await),
            (Some(QuestionStatus::Unanswered), Some(topic), None) => self
                .in_memory_repository
                .get_unanswered_imam_questions_by_topic(&topic)
                .await
                .or(self
                    .repository
                    .get_unanswered_imam_questions_by_topic(&topic)
                    .await),
            (Some(QuestionStatus::Unanswered), None, Some(school_of_thought)) => self
                .in_memory_repository
                .get_unanswered_imam_questions_by_school_of_thought(school_of_thought)
                .await
                .or(self
                    .repository
                    .get_unanswered_imam_questions_by_school_of_thought(school_of_thought)
                    .await),
            (Some(QuestionStatus::Unanswered), Some(topic), Some(school_of_thought)) => self
                .in_memory_repository
                .get_unanswered_imam_questions_by_topic_and_school_of_thought(
                    &topic,
                    school_of_thought,
                )
                .await
                .or(self
                    .repository
                    .get_unanswered_imam_questions_by_topic_and_school_of_thought(
                        &topic,
                        school_of_thought,
                    )
                    .await),
            (Some(QuestionStatus::Answered), None, None) => self
                .in_memory_repository
                .get_answered_questions()
                .await
                .or(self.repository.get_answered_questions().await),
            (Some(QuestionStatus::Answered), Some(topic), None) => self
                .in_memory_repository
                .get_answered_questions_by_topic(&topic)
                .await
                .or(self
                    .repository
                    .get_answered_questions_by_topic(&topic)
                    .await),
            (Some(QuestionStatus::Answered), None, Some(school_of_thought)) => self
                .in_memory_repository
                .get_answered_questions_by_school_of_thought(school_of_thought)
                .await
                .or(self
                    .repository
                    .get_answered_questions_by_school_of_thought(school_of_thought)
                    .await),
            (Some(QuestionStatus::Answered), Some(topic), Some(school_of_thought)) => self
                .in_memory_repository
                .get_answered_questions_by_topic_and_school_of_thought(&topic, school_of_thought)
                .await
                .or(self
                    .repository
                    .get_answered_questions_by_topic_and_school_of_thought(
                        &topic,
                        school_of_thought,
                    )
                    .await),
        }
    }

    async fn provide_answer_to_question(
        &self,
        question_id: i32,
        answer: Answer,
    ) -> Result<(), UpsertAnswerToQuestionError> {
        if self
            .in_memory_repository
            .upsert_imam_answer_to_question(&question_id, &answer)
            .await
            .is_err()
        {
            tracing::warn!(
                question_id = question_id,
                "unable to upsert answer to question for in-memory repository"
            );
        }
        self.repository
            .upsert_imam_answer_to_question(&question_id, &answer)
            .await
    }

    async fn delete_question(&self, question_id: i32) -> Result<(), DeleteQuestionError> {
        if self
            .in_memory_repository
            .delete_imam_question_by_id(&question_id)
            .await
            .is_err()
        {
            tracing::warn!("unable to delete question for in-memory repository");
        }
        self.repository
            .delete_imam_question_by_id(&question_id)
            .await
    }
}
