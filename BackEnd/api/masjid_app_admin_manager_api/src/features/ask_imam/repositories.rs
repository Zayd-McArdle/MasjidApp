use crate::features::ask_imam::errors::{DeleteQuestionError, UpsertAnswerToQuestionError};
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::errors::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::{
    Answer, ImamQuestionDTO, SchoolOfThought,
};
use masjid_app_api_library::features::ask_imam::repositories::{
    get_imam_questions_common, ImamQuestionsRepository,
};
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::data_access::repository_manager::{
    InMemoryRepository, MySqlRepository, RepositoryType,
};
use std::sync::Arc;

#[async_trait]
pub trait ImamQuestionsAdminRepository: ImamQuestionsRepository {
    async fn get_all_imam_questions(&self) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_unanswered_imam_questions(
        &self,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_unanswered_imam_questions_by_topic(
        &self,
        topic: &str,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_unanswered_imam_questions_by_school_of_thought(
        &self,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_unanswered_imam_questions_by_topic_and_school_of_thought(
        &self,
        topic: &str,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn upsert_imam_answer_to_question(
        &self,
        question_id: &i32,
        answer: &Answer,
    ) -> Result<(), UpsertAnswerToQuestionError>;
    async fn delete_imam_question_by_id(&self, id: &i32) -> Result<(), DeleteQuestionError>;
}

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

pub async fn new_imam_questions_admin_repository(
    db_type: DbType,
) -> Arc<dyn ImamQuestionsAdminRepository> {
    match db_type {
        DbType::InMemory => Arc::new(InMemoryRepository::new(RepositoryType::AskImam).await),
        DbType::MySql => Arc::new(MySqlRepository::new(RepositoryType::AskImam).await),
    }
}
