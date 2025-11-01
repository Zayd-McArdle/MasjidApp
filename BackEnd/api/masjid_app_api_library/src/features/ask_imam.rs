use async_trait::async_trait;

pub struct Reference {
    source: String,
    url: String,
}
pub struct Answer {
    pub text: String,
    pub reference: Option<Vec<Reference>>,
}

pub struct QuestionDTO {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub questioner: String,
    pub answer: Option<Answer>,
}

#[async_trait]
pub trait ImamQuestionsRepository: Send + Sync {
    async fn get_questions();
}
