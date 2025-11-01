use async_trait::async_trait;

pub struct Reference {
    source: String,
    url: String,
}
pub struct Answer {
    pub imam_name: String,
    pub text: String,
    pub reference: Option<Vec<Reference>>,
}

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
