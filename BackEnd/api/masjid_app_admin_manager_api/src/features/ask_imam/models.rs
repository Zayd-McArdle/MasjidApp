use enum_stringify::EnumStringify;
use masjid_app_api_library::features::ask_imam::models::Answer;
use serde::Deserialize;
use validator::Validate;
#[derive(Deserialize, EnumStringify)]
#[enum_stringify(case = "lower")]
pub enum QuestionStatus {
    Unanswered,
    Answered,
}

#[derive(Deserialize, Validate)]
pub struct GetImamQuestionsAdminRequest {
    #[validate(length(min = 2))]
    pub topic: Option<String>,

    #[serde(rename = "schoolOfThought")]
    pub school_of_thought: Option<String>,

    #[serde(rename = "questionStatus")]
    pub question_status: Option<String>,
}

#[derive(Validate, Deserialize, Clone)]
pub struct ProvideAnswerForImamQuestionRequest {
    #[validate(range(min = 1))]
    #[serde(rename = "questionID")]
    pub question_id: i32,

    #[validate(length(min = 2))]
    #[serde(rename = "imamName")]
    pub imam_name: String,

    #[validate(length(min = 2))]
    pub text: String,
}

impl Into<Answer> for ProvideAnswerForImamQuestionRequest {
    fn into(self) -> Answer {
        Answer {
            imam_name: self.imam_name,
            text: self.text,
            date_answered: chrono::Utc::now(),
        }
    }
}
