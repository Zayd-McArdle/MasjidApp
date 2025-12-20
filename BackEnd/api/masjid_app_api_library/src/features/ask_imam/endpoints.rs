use crate::features::ask_imam::errors::GetQuestionsError;
use crate::features::ask_imam::models::ImamQuestionDTO;
use crate::features::ask_imam::repositories::ImamQuestionsRepository;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::str::FromStr;
use validator::Validate;

pub fn send_response_for_get_imam_questions(
    get_imam_questions_result: Result<Vec<ImamQuestionDTO>, GetQuestionsError>,
) -> Response {
    match get_imam_questions_result {
        Ok(questions) => (StatusCode::OK, Json(questions)).into_response(),
        Err(GetQuestionsError::QuestionsNotFound) => StatusCode::NO_CONTENT.into_response(),
        Err(GetQuestionsError::UnableToGetAnsweredQuestions) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
mod test {
    use crate::features::ask_imam::endpoints::send_response_for_get_imam_questions;
    use crate::features::ask_imam::errors::GetQuestionsError;
    use crate::features::ask_imam::models::{Answer, ImamQuestionDTO, SchoolOfThought};
    use axum::http::StatusCode;

    fn get_mock_answered_questions() -> Vec<ImamQuestionDTO> {
        vec![
            ImamQuestionDTO {
                id: 1,
                title: "question 1".to_string(),
                topic: "N/A".to_string(),
                school_of_thought: None,
                description: "This is a description".to_string(),
                date_of_question: Default::default(),
                answer: Some(Answer {
                    imam_name: "Zayd".to_string(),
                    text: "This is an answer".to_string(),
                    date_answered: Default::default(),
                }),
            },
            ImamQuestionDTO {
                id: 2,
                title: "question 2".to_string(),
                topic: "N/A".to_string(),
                school_of_thought: None,
                description: "This is a description".to_string(),
                date_of_question: Default::default(),
                answer: Some(Answer {
                    imam_name: "Zayd".to_string(),
                    text: "This is an answer".to_string(),
                    date_answered: Default::default(),
                }),
            },
            ImamQuestionDTO {
                id: 3,
                title: "question 3".to_string(),
                topic: "Specific topic".to_string(),
                school_of_thought: None,
                description: "This is a description".to_string(),
                date_of_question: Default::default(),
                answer: Some(Answer {
                    imam_name: "Zayd".to_string(),
                    text: "This is an answer".to_string(),
                    date_answered: Default::default(),
                }),
            },
            ImamQuestionDTO {
                id: 4,
                title: "question 4".to_string(),
                topic: "N/A".to_string(),
                school_of_thought: Some(SchoolOfThought::Hanafi),
                description: "This is a description".to_string(),
                date_of_question: Default::default(),
                answer: Some(Answer {
                    imam_name: "Zayd".to_string(),
                    text: "This is an answer".to_string(),
                    date_answered: Default::default(),
                }),
            },
        ]
    }
    #[test]
    fn test_send_response_for_get_answered_questions() {
        struct TestCase {
            description: &'static str,
            get_answered_questions_result: Result<Vec<ImamQuestionDTO>, GetQuestionsError>,
            expected_response_code: StatusCode,
        }
        let imam_questions = get_mock_answered_questions();
        let test_cases = [
            TestCase {
                description: "When get_answered_questions_result is okay, I should retrieve answered questions with no error",
                get_answered_questions_result: Ok(imam_questions),
                expected_response_code: StatusCode::OK,
            },
            TestCase {
                description: "When no questions are found, I should get a NO_CONTENT response",
                get_answered_questions_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_response_code: StatusCode::NO_CONTENT,
            },
            TestCase {
                description: "When questions are unable to be retrieved, I should get an INTERNAL_SERVER_ERROR response",
                get_answered_questions_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        ];

        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let actual_response =
                send_response_for_get_imam_questions(test_case.get_answered_questions_result);
            assert_eq!(test_case.expected_response_code, actual_response.status());
        }
    }
}
