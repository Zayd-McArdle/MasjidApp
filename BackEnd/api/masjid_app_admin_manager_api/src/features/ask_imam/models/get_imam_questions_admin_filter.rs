use crate::features::ask_imam::models::{
    get_imam_questions_admin_request::GetImamQuestionsAdminRequest, question_status::QuestionStatus,
};
#[derive(Default)]
pub struct GetImamQuestionsAdminFilter {
    pub topic: Option<String>,
    pub school_of_thought: Option<String>,
    pub question_status: QuestionStatus,
}

impl From<GetImamQuestionsAdminRequest> for GetImamQuestionsAdminFilter {
    fn from(value: GetImamQuestionsAdminRequest) -> Self {
        Self {
            topic: value.topic,
            school_of_thought: value.school_of_thought.map(|s| s.to_string()),
            question_status: value.question_status,
        }
    }
}
