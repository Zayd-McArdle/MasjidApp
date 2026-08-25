use crate::features::ask_imam::errors::delete_question_error::DeleteQuestionError;
use crate::features::ask_imam::errors::upsert_answer_to_question_error::UpsertAnswerToQuestionError;
use crate::features::ask_imam::repositories::ImamQuestionsAdminRepository;
use crate::features::ask_imam::models::get_imam_questions_admin_filter::GetImamQuestionsAdminFilter;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::answer::Answer;
use masjid_app_api_library::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use masjid_app_api_library::features::ask_imam::services::AskImamServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait AskImamAdminService: Send + Sync {
    async fn get_questions(
        &self,
        filter: GetImamQuestionsAdminFilter,
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
        filter: GetImamQuestionsAdminFilter,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        match self.in_memory_repository.get_questions(&filter).await {
            Ok(questions) => Ok(questions),
            Err(_) => self.repository.get_questions(&filter).await,
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
