use crate::features::ask_imam::errors::GetQuestionsError;
use crate::features::ask_imam::models::{ImamQuestion, ImamQuestionDTO, SchoolOfThought};
use crate::shared::data_access::repository_manager::{InMemoryRepository, MySqlRepository};
use async_trait::async_trait;
use mockall::automock;
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool, Row};
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait ImamQuestionsRepository: Send + Sync {
    async fn get_answered_questions(&self) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_answered_questions_by_topic(
        &self,
        topic: &str,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_answered_questions_by_school_of_thought(
        &self,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn get_answered_questions_by_topic_and_school_of_thought(
        &self,
        topic: &str,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
}

fn imam_question_from_my_sql_row(row: MySqlRow) -> ImamQuestion {
    ImamQuestion {
        id: row.get(0),
        title: row.get(1),
        topic: row.get(2),
        school_of_thought: row.get(3),
        description: row.get(4),
        date_of_question: row.get(5),
        imam_name: row.get(6),
        answer: row.get(7),
        date_answered: row.get(8),
    }
}

pub async fn get_imam_questions_common(
    db_connection: Arc<MySqlPool>,
    stored_procedure: &'static str,
    topic_parameter: Option<&str>,
    school_of_thought_parameter: Option<&str>,
) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
    let questions = {
        let mut query = sqlx::query(stored_procedure);
        if let Some(topic) = topic_parameter {
            query = query.bind(topic)
        }
        if let Some(school_of_thought) = school_of_thought_parameter {
            query = query.bind(school_of_thought);
        }
        query
    }
    .map(imam_question_from_my_sql_row)
    .fetch_all(&*db_connection)
    .await
    .map_err(|err| {
        if let sqlx::Error::RowNotFound = err {
            return GetQuestionsError::QuestionsNotFound;
        }
        tracing::error!(
            stored_procedure = stored_procedure,
            error = err.to_string(),
            "unable to fetch questions from imam from database",
        );
        GetQuestionsError::UnableToGetAnsweredQuestions
    })?;
    if questions.is_empty() {
        return Err(GetQuestionsError::QuestionsNotFound);
    }
    Ok(questions.into_iter().map(ImamQuestionDTO::from).collect())
}
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

#[async_trait]
impl ImamQuestionsRepository for MySqlRepository {
    async fn get_answered_questions(&self) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        get_imam_questions_common(
            self.db_connection.clone(),
            "CALL get_answered_imam_questions()",
            None,
            None,
        )
        .await
    }

    async fn get_answered_questions_by_topic(
        &self,
        topic: &str,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        get_imam_questions_common(
            self.db_connection.clone(),
            "CALL get_answered_imam_questions_by_topic(?)",
            Some(topic),
            None,
        )
        .await
    }

    async fn get_answered_questions_by_school_of_thought(
        &self,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        get_imam_questions_common(
            self.db_connection.clone(),
            "CALL get_answered_imam_questions_by_school_of_thought(?)",
            None,
            Some(&school_of_thought.to_string()),
        )
        .await
    }

    async fn get_answered_questions_by_topic_and_school_of_thought(
        &self,
        topic: &str,
        school_of_thought: SchoolOfThought,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        get_imam_questions_common(
            self.db_connection.clone(),
            "CALL get_answered_imam_questions_by_topic_and_school_of_thought(?, ?)",
            Some(topic),
            Some(&school_of_thought.to_string()),
        )
        .await
    }
}
