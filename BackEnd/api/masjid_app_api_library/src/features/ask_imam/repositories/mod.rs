use crate::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use crate::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use crate::features::ask_imam::models::school_of_thought::SchoolOfThought;
use async_trait::async_trait;
use mockall::automock;
pub mod mysql_impl;
mod redis_impl;

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
