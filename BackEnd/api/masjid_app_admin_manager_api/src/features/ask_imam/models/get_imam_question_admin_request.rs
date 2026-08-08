use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct GetImamQuestionsAdminRequest {
    #[validate(length(min = 2))]
    pub topic: Option<String>,

    #[serde(rename = "schoolOfThought")]
    pub school_of_thought: Option<String>,

    #[serde(rename = "questionStatus")]
    pub question_status: Option<String>,
}
