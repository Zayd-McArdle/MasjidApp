use crate::features::ask_imam::errors::delete_question_error::DeleteQuestionError;
use crate::features::ask_imam::errors::upsert_answer_to_question_error::UpsertAnswerToQuestionError;
use crate::features::ask_imam::repositories::ImamQuestionsAdminRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::answer::Answer;
use masjid_app_api_library::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use masjid_app_api_library::features::ask_imam::models::school_of_thought::SchoolOfThought;
use masjid_app_api_library::features::ask_imam::repositories::mysql_impl::get_imam_questions_common;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;

#[async_trait]
impl ImamQuestionsAdminRepository for MySqlRepository {
    async fn get_all_imam_questions(&self) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        get_imam_questions_common(
            self.db_connection.clone(),
            "CALL get_all_imam_questions();",
            None,
            None,
        )
        .await
    }

    async fn get_unanswered_imam_questions(
        &self,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        get_imam_questions_common(
            self.db_connection.clone(),
            "CALL get_unanswered_imam_questions();",
            None,
            None,
        )
        .await
    }

    async fn get_unanswered_imam_questions_by_topic(
        &self,
        topic: &str,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        get_imam_questions_common(
            self.db_connection.clone(),
            "CALL get_unanswered_imam_questions_by_topic(?);",
            Some(topic),
            None,
        )
        .await
    }

    async fn get_unanswered_imam_questions_by_school_of_thought(
        &self,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        get_imam_questions_common(
            self.db_connection.clone(),
            "CALL get_unanswered_imam_questions_by_school_of_thought(?);",
            None,
            Some(&school_of_thought.to_string()),
        )
        .await
    }

    async fn get_unanswered_imam_questions_by_topic_and_school_of_thought(
        &self,
        topic: &str,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        get_imam_questions_common(
            self.db_connection.clone(),
            "CALL get_unanswered_imam_questions_by_topic_and_school_of_thought(?, ?);",
            Some(topic),
            Some(&school_of_thought.to_string()),
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
        let query_result = sqlx::query("CALL upsert_imam_answer_to_question(?, ?, ?, ?)")
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
        let query_result = sqlx::query("CALL delete_imam_question_by_id(?)")
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
