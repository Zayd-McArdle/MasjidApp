use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Reference {
    source: String,
    url: String,
}
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Answer {
    #[serde(rename = "imamName")]
    pub imam_name: String,
    pub text: String,
    pub reference: Option<Vec<Reference>>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct QuestionDTO {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub answer: Option<Answer>,
}

pub enum GetAnsweredQuestionsError {
    AnsweredQuestionsNotFound,
    UnableToGetAnsweredQuestions,
}

#[async_trait]
pub trait ImamQuestionsRepository: Send + Sync {
    async fn get_answered_questions() -> Result<Vec<QuestionDTO>, GetAnsweredQuestionsError>;
}
