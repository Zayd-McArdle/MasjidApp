use crate::features::ask_imam::models::get_imam_questions_request::GetImamQuestionsRequest;
#[derive(Default)]
pub struct GetImamQuestionsFilter {
    pub topic: Option<String>,
    pub school_of_thought: Option<String>,
}

impl From<GetImamQuestionsRequest> for GetImamQuestionsFilter {
    #[inline]
    fn from(value: GetImamQuestionsRequest) -> Self {
        Self {
            topic: value.topic,
            school_of_thought: value
                .school_of_thought
                .map(move |school| school.to_string()),
        }
    }
}
