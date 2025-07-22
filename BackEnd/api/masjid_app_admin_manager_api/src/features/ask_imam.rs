use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam;
use masjid_app_api_library::features::ask_imam::QuestionDTO;

#[async_trait]
pub trait ImamQuestionsAdminRepository {
    async fn get_questions();
    async fn insert_answer_to_question(question: QuestionDTO);
    async fn update_answer_to_question(question: QuestionDTO);
    async fn delete_question_by_id(id: i64);
}