use crate::features::ask_imam::errors::delete_question_error::DeleteQuestionError;
use crate::features::ask_imam::services::AskImamAdminService;
use crate::shared::jwt::Claims;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

pub async fn delete_imam_question(
    State(state): State<ServiceAppState<Arc<dyn AskImamAdminService>>>,
    _claims: Claims,
    Path(question_id): Path<i32>,
) -> Result<(), StatusCode> {
    if question_id == 0 {
        return Err(StatusCode::BAD_REQUEST);
    }
    state
        .service
        .delete_question(question_id)
        .await
        .map_err(move |err| match err {
            DeleteQuestionError::QuestionNotFound => StatusCode::NOT_FOUND,
            DeleteQuestionError::UnableToDeleteQuestion => StatusCode::INTERNAL_SERVER_ERROR,
        })
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
            expected_result: Result<(), StatusCode>,
        }
        let test_cases = [
            TestCase {
                description: "When the JSON request is invalid, I should get a BAD_REQUEST response",
                question_id: 0,
                expected_db_response: None,
                expected_result: Err(StatusCode::BAD_REQUEST),
            },
            TestCase {
                description: "When deleting a non-existent question, I should get a NOT_FOUND response",
                question_id: 1,
                expected_db_response: Some(Err(DeleteQuestionError::QuestionNotFound)),
                expected_result: Err(StatusCode::NOT_FOUND),
            },
            TestCase {
                description: "When deletion fails, I should get an INTERNAL_SERVER_ERROR response",
                question_id: 1,
                expected_db_response: Some(Err(DeleteQuestionError::UnableToDeleteQuestion)),
                expected_result: Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            TestCase {
                description: "When deletion succeeds, I should get an OK response",
                question_id: 1,
                expected_db_response: Some(Ok(())),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockAskImamAdminService::new();
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_service
                    .expect_delete_question()
                    .returning(move |_| expected_db_response);
            }
            let arc_respository: Arc<dyn AskImamAdminService> = Arc::new(mock_service);
            let app_state = ServiceAppState {
                service: arc_respository,
            };
            let actual_result = delete_imam_question(
                State(app_state),
                Claims::default(),
                Path(test_case.question_id),
            )
            .await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
