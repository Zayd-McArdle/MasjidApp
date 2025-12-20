use crate::features::ask_imam::errors::InsertImamQuestionError;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::errors::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::ImamQuestion;
use masjid_app_api_library::features::ask_imam::repositories::ImamQuestionsRepository;
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::data_access::repository_manager::{
    InMemoryRepository, MySqlRepository, RepositoryType,
};
use std::sync::Arc;
use sqlx::Error;

#[async_trait]
pub trait ImamQuestionsPublicRepository: ImamQuestionsRepository {
    async fn insert_question_for_imam(
        &self,
        questions: &ImamQuestion,
    ) -> Result<(), InsertImamQuestionError>;
}

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

pub async fn new_imam_questions_public_repository(
    db_type: DbType,
) -> Arc<dyn ImamQuestionsPublicRepository> {
    match db_type {
        DbType::InMemory => Arc::new(InMemoryRepository::new(RepositoryType::AskImam).await),
        DbType::MySql => Arc::new(MySqlRepository::new(RepositoryType::AskImam).await),
    }
}
