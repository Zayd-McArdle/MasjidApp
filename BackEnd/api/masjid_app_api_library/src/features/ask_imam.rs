use std::sync::Arc;
use async_trait::async_trait;
use crate::shared::app_state::DbType;
use crate::shared::repository_manager::{InMemoryRepository, MySqlRepository, RepositoryType};

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
    async fn insert_question(question: QuestionDTO);
    async fn get_questions();
}

pub async fn new_imam_questions_repository(db_type: DbType) -> Arc<dyn ImamQuestionsRepository> {
    match db_type {
        DbType::InMemory => Arc::new(InMemoryRepository::new(RepositoryType::AskImam).await),
        DbType::MySql => Arc::new(MySqlRepository::new(RepositoryType::AskImam).await),
    }
}