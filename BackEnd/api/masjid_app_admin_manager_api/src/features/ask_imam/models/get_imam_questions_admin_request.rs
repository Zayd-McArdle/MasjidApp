use masjid_app_api_library::features::ask_imam::models::school_of_thought::SchoolOfThought;
use serde::Deserialize;
use validator::Validate;

use crate::features::ask_imam::models::question_status::QuestionStatus;

#[derive(Deserialize, Validate)]
pub struct GetImamQuestionsAdminRequest {
    #[validate(length(min = 2))]
    pub topic: Option<String>,

    #[serde(rename = "schoolOfThought")]
    pub school_of_thought: Option<SchoolOfThought>,

    #[serde(rename = "questionStatus")]
    pub question_status: QuestionStatus,
}
