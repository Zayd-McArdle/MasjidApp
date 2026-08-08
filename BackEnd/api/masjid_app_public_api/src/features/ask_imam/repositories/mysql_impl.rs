use crate::features::ask_imam::errors::insert_imam_question_error::InsertImamQuestionError;
use crate::features::ask_imam::repositories::ImamQuestionsPublicRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::models::imam_question::ImamQuestion;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;

#[async_trait]
impl ImamQuestionsPublicRepository for MySqlRepository {
    async fn insert_question_for_imam(
        &self,
        questions: &ImamQuestion,
    ) -> Result<(), InsertImamQuestionError> {
        let db_connection = self.db_connection.clone();
        let query_result = sqlx::query("CALL insert_question_for_imam(?, ?, ?, ?, ?);")
            .bind(&questions.title)
            .bind(&questions.topic)
            .bind(&questions.school_of_thought)
            .bind(&questions.description)
            .bind(&questions.date_of_question)
            .execute(&*db_connection)
            .await
            .map_err(|err| {
                tracing::error!(
                    error = err.to_string(),
                    "unable to insert question for imam into database"
                );
                InsertImamQuestionError::UnableToInsertQuestion
            })?;
        if query_result.rows_affected() == 0 {
            return Err(InsertImamQuestionError::UnableToInsertQuestion);
        }
        Ok(())
    }
}
