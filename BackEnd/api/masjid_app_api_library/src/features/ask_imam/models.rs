use enum_stringify::EnumStringify;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::str::FromStr;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct GetImamQuestionsRequest {
    #[validate(length(min = 2))]
    pub topic: Option<String>,

    #[serde(rename = "schoolOfThought")]
    pub school_of_thought: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, EnumStringify, Clone, Copy, Eq, PartialEq)]
pub enum SchoolOfThought {
    Hanafi,
    Shaafi,
    Maliki,
    Hanbali,
}
#[derive(Debug, Deserialize, Serialize, Validate, Clone, Eq, PartialEq)]
pub struct Answer {
    #[serde(rename = "imamName")]
    pub imam_name: String,

    pub text: String,
    #[serde(rename = "dateAnswered")]
    pub date_answered: chrono::DateTime<chrono::Utc>,
}

#[derive(FromRow, Debug, Clone)]
pub struct ImamQuestion {
    pub id: i32,

    pub title: String,

    pub topic: String,

    pub school_of_thought: Option<String>,

    pub description: String,

    pub date_of_question: chrono::DateTime<chrono::Utc>,

    pub imam_name: Option<String>,

    pub answer: Option<String>,

    pub date_answered: Option<chrono::DateTime<chrono::Utc>>,
}
impl From<ImamQuestionDTO> for ImamQuestion {
    fn from(dto: ImamQuestionDTO) -> Self {
        let mut imam_name = None;
        let mut imam_answer = None;
        let mut date_answered = None;
        if let Some(answer) = dto.answer {
            imam_name = Some(answer.imam_name);
            imam_answer = Some(answer.text);
            date_answered = Some(answer.date_answered);
        }
        ImamQuestion {
            id: dto.id,
            title: dto.title,
            topic: dto.topic,
            school_of_thought: dto
                .school_of_thought
                .and_then(|value| Some(value.to_string())),
            description: dto.description,
            date_of_question: dto.date_of_question,
            imam_name: imam_name,
            answer: imam_answer,
            date_answered: date_answered,
        }
    }
}

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
