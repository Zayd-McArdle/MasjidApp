use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct GetImamQuestionsRequest {
    #[validate(length(min = 2))]
    pub topic: Option<String>,

    #[serde(rename = "schoolOfThought")]
    pub school_of_thought: Option<String>,
}
