use crate::features::ask_imam::errors::delete_question_error::DeleteQuestionError;
use crate::features::ask_imam::errors::upsert_answer_to_question_error::UpsertAnswerToQuestionError;
use crate::features::ask_imam::models::get_imam_questions_admin_filter::GetImamQuestionsAdminFilter;
use crate::features::ask_imam::models::question_status::QuestionStatus;
use crate::features::ask_imam::repositories::ImamQuestionsAdminRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::answer::Answer;
use masjid_app_api_library::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use masjid_app_api_library::features::ask_imam::repositories::mysql_impl::get_imam_questions_common;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;

const GET_ALL_IMAM_QUESTIONS_STORED_PROCEDURE: &str = "CALL get_all_imam_questions(?, ?)";
const GET_UNANSWERED_IMAM_QUESTIONS_STORED_PROCEDURE: &str =
    "CALL get_unanswered_imam_questions(?, ?)";
const GET_ANSWERED_IMAM_QUESTIONS_STORED_PROCEDURE: &str = "CALL get_answered_imam_questions(?, ?)";
const DELETE_IMAM_QUESTION_BY_ID_STORED_PROCEDURE: &str = "CALL delete_imam_question_by_id(?)";
const UPSERT_IMAM_ANSWER_TO_QUESTION_STORED_PROCEDURE: &str =
    "CALL upsert_imam_answer_to_question(?, ?, ?, ?)";

#[async_trait]
impl ImamQuestionsAdminRepository for MySqlRepository {
    async fn get_questions(
        &self,
        filter: &GetImamQuestionsAdminFilter,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        let query = match &filter.question_status {
            &QuestionStatus::Unanswered => GET_UNANSWERED_IMAM_QUESTIONS_STORED_PROCEDURE,
            &QuestionStatus::Answered => GET_ANSWERED_IMAM_QUESTIONS_STORED_PROCEDURE,
            &QuestionStatus::All => GET_ALL_IMAM_QUESTIONS_STORED_PROCEDURE,
        };
        get_imam_questions_common(
            self.db_connection.clone(),
            query,
            filter.topic.clone(),
            filter.school_of_thought.clone(),
        )
        .await
    }

    async fn upsert_imam_answer_to_question(
        &self,
        question_id: &i32,
        answer: &Answer,
    ) -> Result<(), UpsertAnswerToQuestionError> {
        tracing::debug!(
            question_id = question_id,
            "upserting imam's answer to question in database"
        );
        let db_connection = self.db_connection.clone();
        let query_result = sqlx::query(UPSERT_IMAM_ANSWER_TO_QUESTION_STORED_PROCEDURE)
            .bind(&answer.imam_name)
            .bind(&answer.text)
            .bind(&answer.date_answered)
            .bind(question_id)
            .execute(&*db_connection)
            .await
            .map_err(|err| {
                tracing::error!(
                    stored_procedure = "upsert_imam_answer_to_question",
                    error = err.to_string(),
                    "unable to upsert imam answer to question in database"
                );
                UpsertAnswerToQuestionError::UnableToUpsertAnswerToQuestion
            })?;
        if query_result.rows_affected() == 0 {
            return Err(UpsertAnswerToQuestionError::QuestionNotFound);
        }
        Ok(())
    }

    async fn delete_imam_question_by_id(&self, id: &i32) -> Result<(), DeleteQuestionError> {
        let db_connection = self.db_connection.clone();
        tracing::debug!(question_id = id, "deleting question from database");
        let query_result = sqlx::query(DELETE_IMAM_QUESTION_BY_ID_STORED_PROCEDURE)
            .bind(id)
            .execute(&*db_connection)
            .await
            .map_err(|err| {
                tracing::error!(
                    stored_prcedure = "delete_imam_question_by_id",
                    question_id = id,
                    error = err.to_string(),
                    "unable to delete question from database"
                );
                DeleteQuestionError::UnableToDeleteQuestion
            })?;
        if query_result.rows_affected() == 0 {
            return Err(DeleteQuestionError::QuestionNotFound);
        }
        tracing::debug!(
            question_id = id,
            "successfully deleted question from database"
        );
        Ok(())
    }
}
