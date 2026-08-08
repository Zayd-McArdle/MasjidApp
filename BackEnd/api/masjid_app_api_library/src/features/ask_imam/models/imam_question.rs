use crate::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use sqlx::FromRow;
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
        Self {
            id: dto.id,
            title: dto.title,
            topic: dto.topic,
            school_of_thought: dto
                .school_of_thought
                .and_then(|value| Some(value.to_string())),
            description: dto.description,
            date_of_question: dto.date_of_question,
            imam_name,
            answer: imam_answer,
            date_answered,
        }
    }
}
