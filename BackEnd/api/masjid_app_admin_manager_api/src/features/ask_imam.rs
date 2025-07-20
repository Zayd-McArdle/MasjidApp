use async_trait::async_trait;

#[async_trait]
pub trait ImamQuestionsAdminRepository : ImamQuestionsRepository {
    async fn insert_answer_to_question(question: QuestionDTO);
    async fn update_answer_to_question(question: QuestionDTO);
    async fn delete_question_by_id(id: i64);
}