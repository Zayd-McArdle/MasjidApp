use crate::features::ask_imam::errors::insert_imam_question_error::InsertImamQuestionError;
use crate::features::ask_imam::models::ask_imam_request::AskImamRequest;
use crate::features::ask_imam::services::AskImamPublicService;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

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
mod tests {
    use super::*;
    use crate::features::ask_imam::services::{AskImamPublicService, MockAskImamPublicService};

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
