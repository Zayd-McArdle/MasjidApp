use crate::features::ask_imam::errors::delete_question_error::DeleteQuestionError;
use crate::features::ask_imam::services::AskImamAdminService;
use crate::shared::jwt::Claims;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

pub async fn delete_imam_question(
    State(state): State<ServiceAppState<Arc<dyn AskImamAdminService>>>,
    _claims: Claims,
    Path(questions_id): Path<i32>,
) -> Response {
    if questions_id == 0 {
        return (StatusCode::BAD_REQUEST, "question ids cannot be 0").into_response();
    }
    match state.service.delete_question(questions_id).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(DeleteQuestionError::QuestionNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(DeleteQuestionError::UnableToDeleteQuestion) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::features::ask_imam::services::MockAskImamAdminService;
    #[tokio::test]
    async fn test_delete_imam_question() {
        struct TestCase {
            description: &'static str,
            question_id: i32,
            expected_db_response: Option<Result<(), DeleteQuestionError>>,
            expected_status_code: StatusCode,
        }
        let test_cases = [
            TestCase {
                description: "When the JSON request is invalid, I should get a BAD_REQUEST response",
                question_id: 0,
                expected_db_response: None,
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "When deleting a non-existent question, I should get a NOT_FOUND response",
                question_id: 1,
                expected_db_response: Some(Err(DeleteQuestionError::QuestionNotFound)),
                expected_status_code: StatusCode::NOT_FOUND,
            },
            TestCase {
                description: "When deletion fails, I should get an INTERNAL_SERVER_ERROR response",
                question_id: 1,
                expected_db_response: Some(Err(DeleteQuestionError::UnableToDeleteQuestion)),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "When deletion succeeds, I should get an OK response",
                question_id: 1,
                expected_db_response: Some(Ok(())),
                expected_status_code: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockAskImamAdminService::new();
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_service
                    .expect_delete_question()
                    .return_once(move |_| expected_db_response);
            }
            let arc_respository: Arc<dyn AskImamAdminService> = Arc::new(mock_service);
            let app_state = ServiceAppState {
                service: arc_respository,
            };
            let actual_response = delete_imam_question(
                State(app_state),
                Claims::default(),
                Path(test_case.question_id),
            )
            .await;
            assert_eq!(test_case.expected_status_code, actual_response.status());
        }
    }
}
