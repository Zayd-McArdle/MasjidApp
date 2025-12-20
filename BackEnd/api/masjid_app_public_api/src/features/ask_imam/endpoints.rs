use crate::features::ask_imam::errors::InsertImamQuestionError;
use crate::features::ask_imam::models::AskImamRequest;
use crate::features::ask_imam::services::AskImamPublicService;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use masjid_app_api_library::features::ask_imam::endpoints::send_response_for_get_imam_questions;
use masjid_app_api_library::features::ask_imam::models::{
    GetImamQuestionsRequest, SchoolOfThought,
};
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::str::FromStr;
use std::sync::Arc;
use validator::Validate;

pub async fn get_answered_questions(
    State(state): State<ServiceAppState<Arc<dyn AskImamPublicService>>>,
    Query(request): Query<GetImamQuestionsRequest>,
) -> Response
where
{
    if request.validate().is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let get_answered_questions_result = state
        .service
        .get_answered_questions(
            request.topic,
            request
                .school_of_thought
                .and_then(|school| SchoolOfThought::from_str(&school).ok()),
        )
        .await;
    send_response_for_get_imam_questions(get_answered_questions_result)
}

pub async fn ask_question_for_imam(
    State(state): State<ServiceAppState<Arc<dyn AskImamPublicService>>>,
    Json(request): Json<AskImamRequest>,
) -> Response {
    if request.validate().is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    match state.service.ask_question(request.into()).await {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(InsertImamQuestionError::UnableToInsertQuestion) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::features::ask_imam::endpoints::ask_question_for_imam;
    use crate::features::ask_imam::services::MockAskImamPublicService;
    use masjid_app_api_library::features::ask_imam::errors::GetQuestionsError;
    use masjid_app_api_library::features::ask_imam::models::{Answer, ImamQuestionDTO};
    use masjid_app_api_library::shared::types::app_state::ServiceAppState;

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

    #[tokio::test]
    async fn test_get_answered_questions() {
        struct TestCase {
            description: &'static str,
            request: GetImamQuestionsRequest,
            expected_service_result: Option<Result<Vec<ImamQuestionDTO>, GetQuestionsError>>,
            expected_response_code: StatusCode,
        }
        let test_cases = [
            TestCase {
                description: "When I use an invalid request, I should get a BAD_REQUEST response",
                request: GetImamQuestionsRequest {
                    topic: Some("".to_owned()),
                    school_of_thought: Some("invalid school of thought".to_owned()),
                },
                expected_service_result: None,
                expected_response_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "When the service fails to retrieve questions, I should receive an INTERNAL_SERVER_ERROR response",
                request: GetImamQuestionsRequest {
                    topic: None,
                    school_of_thought: None,
                },
                expected_service_result: Some(Err(GetQuestionsError::UnableToGetAnsweredQuestions)),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "When the service returns no questions, I should receive a NO_CONTENT response",
                request: GetImamQuestionsRequest {
                    topic: None,
                    school_of_thought: None,
                },
                expected_service_result: Some(Err(GetQuestionsError::QuestionsNotFound)),
                expected_response_code: StatusCode::NO_CONTENT,
            },
            TestCase {
                description: "When the service returns questions, I should receive an OK response",
                request: GetImamQuestionsRequest {
                    topic: None,
                    school_of_thought: None,
                },
                expected_service_result: Some(Ok(get_mock_answered_questions())),
                expected_response_code: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_ask_imam_service = MockAskImamPublicService::new();
            if let Some(expected_service_result) = test_case.expected_service_result {
                mock_ask_imam_service
                    .expect_get_answered_questions()
                    .return_once(move |_, _| expected_service_result);
            }
            let app_state = ServiceAppState::<Arc<dyn AskImamPublicService>> {
                service: Arc::new(mock_ask_imam_service),
            };
            let actual_result =
                get_answered_questions(State(app_state), Query(test_case.request)).await;
            assert_eq!(test_case.expected_response_code, actual_result.status());
        }
    }

    #[tokio::test]
    async fn test_ask_question_for_imam() {
        struct TestCase {
            description: &'static str,
            request: AskImamRequest,
            expected_service_result: Option<Result<(), InsertImamQuestionError>>,
            expected_status_code: StatusCode,
        }
        let test_cases = [
            TestCase {
                description: "When the request is not valid I should get a BAD_REQUEST response",
                request: AskImamRequest {
                    title: "".to_string(),
                    topic: "".to_string(),
                    school_of_thought: None,
                    description: "".to_string(),
                },
                expected_service_result: None,
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "When insertion fails, I should get an INTERNAL_SERVER_ERROR response",
                request: AskImamRequest {
                    title: "title".to_string(),
                    topic: "topic".to_string(),
                    school_of_thought: None,
                    description: "description".to_string(),
                },
                expected_service_result: Some(Err(InsertImamQuestionError::UnableToInsertQuestion)),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "When insertion succeeds, I should get a CREATED response",
                request: AskImamRequest {
                    title: "title".to_string(),
                    topic: "topic".to_string(),
                    school_of_thought: None,
                    description: "description".to_string(),
                },
                expected_service_result: Some(Ok(())),
                expected_status_code: StatusCode::CREATED,
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockAskImamPublicService::new();
            if let Some(expected_service_result) = test_case.expected_service_result {
                mock_service
                    .expect_ask_question()
                    .return_once(move |_| expected_service_result);
            }
            let app_state = ServiceAppState::<Arc<dyn AskImamPublicService>> {
                service: Arc::new(mock_service),
            };
            let actual_response =
                ask_question_for_imam(State(app_state), Json(test_case.request)).await;
            assert_eq!(test_case.expected_status_code, actual_response.status());
        }
    }
}
