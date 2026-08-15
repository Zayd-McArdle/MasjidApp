use crate::features::ask_imam::errors::upsert_answer_to_question_error::UpsertAnswerToQuestionError;
use crate::features::ask_imam::models::provide_answer_for_imam_question_request::ProvideAnswerForImamQuestionRequest;
use crate::features::ask_imam::services::AskImamAdminService;
use crate::shared::jwt::Claims;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

pub async fn provide_answer_for_imam_question(
    State(state): State<ServiceAppState<Arc<dyn AskImamAdminService>>>,
    claims: Claims,
    Json(request): Json<ProvideAnswerForImamQuestionRequest>,
) -> Response {
    if request.validate().is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    match state
        .service
        .provide_answer_to_question(request.question_id, request.into())
        .await
    {
        Ok(()) => StatusCode::OK.into_response(),
        Err(UpsertAnswerToQuestionError::QuestionNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(UpsertAnswerToQuestionError::UnableToUpsertAnswerToQuestion) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::ask_imam::errors::upsert_answer_to_question_error::UpsertAnswerToQuestionError;
    use crate::features::ask_imam::services::MockAskImamAdminService;

    #[tokio::test]
    async fn test_provide_answer_for_imam_question() {
        struct TestCase {
            description: &'static str,
            request: ProvideAnswerForImamQuestionRequest,
            expected_db_response: Option<Result<(), UpsertAnswerToQuestionError>>,
            expected_status_code: StatusCode,
        }
        let valid_request = ProvideAnswerForImamQuestionRequest {
            question_id: 1,
            imam_name: "Zayd".to_owned(),
            text: "This is a test answer".to_owned(),
        };
        let test_cases = [
            TestCase {
                description: "When the JSON request is invalid, I should get a BAD_REQUEST response",
                request: ProvideAnswerForImamQuestionRequest {
                    question_id: 0,
                    imam_name: "".to_owned(),
                    text: "".to_owned(),
                },
                expected_db_response: None,
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "When upserting an answer to a non-existent question, I should get a NOT_FOUND response",
                request: valid_request.clone(),
                expected_db_response: Some(Err(UpsertAnswerToQuestionError::QuestionNotFound)),
                expected_status_code: StatusCode::NOT_FOUND,
            },
            TestCase {
                description: "When upsertion fails, I should get an INTERNAL_SERVER_ERROR response",
                request: valid_request.clone(),
                expected_db_response: Some(Err(
                    UpsertAnswerToQuestionError::UnableToUpsertAnswerToQuestion,
                )),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "When upsertion succeeds, I should get an OK response",
                request: valid_request,
                expected_db_response: Some(Ok(())),
                expected_status_code: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockAskImamAdminService::new();
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_service
                    .expect_provide_answer_to_question()
                    .return_once(move |_, _| expected_db_response);
            }
            let arc_service: Arc<dyn AskImamAdminService> = Arc::new(mock_service);
            let app_state = ServiceAppState {
                service: arc_service,
            };
            let actual_response = provide_answer_for_imam_question(
                State(app_state),
                Claims::default(),
                Json(test_case.request),
            )
            .await;
            assert_eq!(test_case.expected_status_code, actual_response.status());
        }
    }
}
