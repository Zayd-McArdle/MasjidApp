use masjid_app_api_library::features::ask_imam::models::{ImamQuestion, SchoolOfThought};
use serde::Deserialize;
use sqlx::types::chrono;
use std::convert::Into;
use validator::Validate;
#[derive(Debug, Deserialize, Validate)]
pub struct AskImamRequest {
    #[validate(length(min = 4))]
    pub title: String,

    #[validate(length(min = 4))]
    pub topic: String,

    #[serde(rename = "schoolOfThought")]
    pub school_of_thought: Option<SchoolOfThought>,

    pub description: String,
}
impl Into<ImamQuestion> for AskImamRequest {
    fn into(self) -> ImamQuestion {
        ImamQuestion {
            id: 0,
            title: self.title,
            topic: self.topic,
            school_of_thought: self
                .school_of_thought
                .and_then(|value| Some(value.to_string())),
            description: self.description,
            date_of_question: chrono::Utc::now(),
            imam_name: None,
            answer: None,
            date_answered: None,
        }
    }
}
