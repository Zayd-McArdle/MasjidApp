use crate::features::feedback::models::feedback_deletion_mode::FeedbackDeletionMode;
use crate::features::feedback::repositories::errors::delete_feedback_by_id_repository_error::DeleteFeedbackByIdRepositoryError;
use crate::features::feedback::services::errors::delete_feedback_service_error::DeleteFeedbackServiceError;
use crate::features::feedback::services::feedback_deletion_service::FeedbackDeletionService;
use crate::shared::jwt::Claims;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
#[inline]
pub async fn delete_feedback_by_id(
    State(state): State<ServiceAppState<Arc<dyn FeedbackDeletionService>>>,
    _claims: Claims,
    Path(feedback_id): Path<u64>,
) -> Result<(), StatusCode> {
    tracing::debug!("Incoming DELETE request to /feedback/{feedback_id}");
    if feedback_id == 0 {
        return Err(StatusCode::BAD_REQUEST);
    }
    state
        .service
        .delete_feedback(FeedbackDeletionMode::Single(feedback_id))
        .await
        .map_err(move |err| match err {
            DeleteFeedbackServiceError::RepositorySingleDeletionError(
                DeleteFeedbackByIdRepositoryError::FeedbackIdNotFound,
            ) => StatusCode::NOT_FOUND,
            DeleteFeedbackServiceError::RepositorySingleDeletionError(
                DeleteFeedbackByIdRepositoryError::UnableToDeleteFeedbackId,
            ) => StatusCode::INTERNAL_SERVER_ERROR,
            DeleteFeedbackServiceError::RepositoryMultiDeletionError(repository_error) => {
                tracing::error!(?repository_error, "Unexpected error deleting feedback");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::feedback::services::feedback_deletion_service::MockFeedbackDeletionService;

    #[tokio::test]
    async fn test_delete_feedback_by_id() {
        struct TestCase {
            description: &'static str,
            id: u64,
            mock_service_result: Option<Result<(), DeleteFeedbackServiceError>>,
            expected_result: Result<(), StatusCode>,
        }

        let test_cases = [
            TestCase {
                description: "When an invalid id is entered, I should get a BAD_REQUEST response",
                id: 0,
                mock_service_result: None,
                expected_result: Err(StatusCode::BAD_REQUEST),
            },
            TestCase {
                description: "When deleting a non-existent id, I should get a NOT_FOUND response",
                id: 1,
                mock_service_result: Some(Err(
                    DeleteFeedbackServiceError::RepositorySingleDeletionError(
                        DeleteFeedbackByIdRepositoryError::FeedbackIdNotFound,
                    ),
                )),

                expected_result: Err(StatusCode::NOT_FOUND),
            },
            TestCase {
                description: "When deletion of feedback by id fails, I should get an INTERNAL_SERVER_ERROR response",
                id: 1,
                mock_service_result: Some(Err(
                    DeleteFeedbackServiceError::RepositorySingleDeletionError(
                        DeleteFeedbackByIdRepositoryError::UnableToDeleteFeedbackId,
                    ),
                )),
                expected_result: Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
        ];

        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockFeedbackDeletionService::new();

            if let Some(mock_service_result) = test_case.mock_service_result {
                mock_service
                    .expect_delete_feedback()
                    .return_once(move |_| mock_service_result);
            }
            let arc_service: Arc<dyn FeedbackDeletionService> = Arc::new(mock_service);
            let app_state = ServiceAppState {
                service: arc_service,
            };
            let actual_result =
                delete_feedback_by_id(State(app_state), Claims::default(), Path(test_case.id))
                    .await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
