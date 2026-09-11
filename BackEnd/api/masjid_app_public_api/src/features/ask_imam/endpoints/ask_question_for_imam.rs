use crate::features::ask_imam::errors::insert_imam_question_error::InsertImamQuestionError;
use crate::features::ask_imam::models::ask_imam_request::AskImamRequest;
use crate::features::ask_imam::services::AskImamPublicService;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

pub async fn ask_question_for_imam(
    State(state): State<ServiceAppState<Arc<dyn AskImamPublicService>>>,
    Json(request): Json<AskImamRequest>,
) -> Result<(), StatusCode> {
    request.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    state
        .service
        .ask_question(request.into())
        .await
        .map_err(move |err| match err {
            InsertImamQuestionError::UnableToInsertQuestion => StatusCode::INTERNAL_SERVER_ERROR,
        })
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
            expected_result: Result<(), StatusCode>,
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
                expected_result: Err(StatusCode::BAD_REQUEST),
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
                expected_result: Err(StatusCode::INTERNAL_SERVER_ERROR),
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
                expected_result: Ok(()),
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
            let actual_result =
                ask_question_for_imam(State(app_state), Json(test_case.request)).await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
