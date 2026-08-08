use crate::features::ask_imam::models::answer::Answer;
use crate::features::ask_imam::models::imam_question::ImamQuestion;
use crate::features::ask_imam::models::school_of_thought::SchoolOfThought;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone, Eq, PartialEq)]
pub struct ImamQuestionDTO {
    pub id: i32,

    pub title: String,

    pub topic: String,

    #[serde(rename = "schoolOfThought")]
    pub school_of_thought: Option<SchoolOfThought>,

    pub description: String,

    #[serde(rename = "dateOfQuestion")]
    pub date_of_question: chrono::DateTime<chrono::Utc>,

    pub answer: Option<Answer>,
}

impl From<ImamQuestion> for ImamQuestionDTO {
    fn from(imam_question: ImamQuestion) -> Self {
        let mut answer = None;
        if let Some(imam_name) = imam_question.imam_name
            && let Some(imam_answer) = imam_question.answer
            && let Some(date_answered) = imam_question.date_answered
        {
            answer = Some(Answer {
                imam_name: imam_name,
                text: imam_answer,
                date_answered: date_answered,
            })
        }
        ImamQuestionDTO {
            id: imam_question.id,
            title: imam_question.title,
            topic: imam_question.topic,
            school_of_thought: imam_question
                .school_of_thought
                .and_then(|value| SchoolOfThought::from_str(&value).ok()),
            description: imam_question.description,
            date_of_question: imam_question.date_of_question,
            answer: answer,
        }
    }
}
