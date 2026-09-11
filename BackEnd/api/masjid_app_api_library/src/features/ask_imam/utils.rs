use crate::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use crate::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use axum::Json;
use axum::http::StatusCode;
#[inline]
pub fn send_response_for_get_imam_questions(
    get_imam_questions_result: Result<Vec<ImamQuestionDTO>, GetQuestionsError>,
) -> Result<Json<Vec<ImamQuestionDTO>>, StatusCode> {
    match get_imam_questions_result {
        Ok(questions) => Ok(Json(questions)),
        Err(GetQuestionsError::QuestionsNotFound) => Err(StatusCode::NOT_FOUND),
        Err(GetQuestionsError::UnableToGetAnsweredQuestions) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_endpoint_json_response;
    use crate::features::ask_imam::models::answer::Answer;
    use crate::features::ask_imam::models::school_of_thought::SchoolOfThought;

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
            expected_result: Result<Json<Vec<ImamQuestionDTO>>, StatusCode>,
        }
        let imam_questions = get_mock_answered_questions();
        let test_cases = [
            TestCase {
                description: "When get_answered_questions_result is okay, I should retrieve answered questions with no error",
                get_answered_questions_result: Ok(imam_questions.clone()),
                expected_result: Ok(Json(imam_questions)),
            },
            TestCase {
                description: "When no questions are found, I should get a NO_CONTENT response",
                get_answered_questions_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(StatusCode::NOT_FOUND),
            },
            TestCase {
                description: "When questions are unable to be retrieved, I should get an INTERNAL_SERVER_ERROR response",
                get_answered_questions_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
        ];

        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let actual_result =
                send_response_for_get_imam_questions(test_case.get_answered_questions_result);
            assert_endpoint_json_response!(test_case.expected_result, actual_result);
        }
    }
}
