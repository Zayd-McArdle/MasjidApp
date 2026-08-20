use crate::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use crate::features::ask_imam::models::get_imam_questions_filter::GetImamQuestionsFilter;
use crate::features::ask_imam::models::imam_question::ImamQuestion;
use crate::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use crate::features::ask_imam::repositories::ImamQuestionsRepository;
use crate::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use async_trait::async_trait;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};
use std::sync::Arc;

#[inline]
pub fn imam_question_from_my_sql_row(row: MySqlRow) -> ImamQuestion {
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
    topic_parameter: Option<String>,
    school_of_thought_parameter: Option<String>,
) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
    let questions = sqlx::query(stored_procedure)
        .bind(topic_parameter)
        .bind(school_of_thought_parameter)
        .map(imam_question_from_my_sql_row)
        .map(ImamQuestionDTO::from)
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
    Ok(questions)
}

#[async_trait]
impl ImamQuestionsRepository for MySqlRepository {
    async fn get_questions(
        &self,
        filter: &GetImamQuestionsFilter,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        get_imam_questions_common(
            self.db_connection.clone(),
            "CALL get_answered_imam_questions(?, ?)",
            filter.topic.clone(),
            filter.school_of_thought.clone(),
        )
        .await
    }
}
