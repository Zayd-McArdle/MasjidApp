use crate::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use crate::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use crate::features::ask_imam::models::get_imam_questions_filter::GetImamQuestionsFilter;
use async_trait::async_trait;
use mockall::automock;
pub mod mysql_impl;
mod redis_impl;

#[automock]
#[async_trait]
pub trait ImamQuestionsRepository: Send + Sync {
    async fn get_questions(&self, filter: &GetImamQuestionsFilter) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
}
