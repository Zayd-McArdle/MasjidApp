use crate::features::feedback::endpoints::delete_feedback_by_id::delete_feedback_by_id;
use crate::features::feedback::models::feedback_deletion_mode::FeedbackDeletionMode;
use crate::features::feedback::models::requests::delete_feedback_request::DeleteFeedbackRequest;
use crate::features::feedback::repositories::errors::delete_feedback_by_id_repository_error::DeleteFeedbackByIdRepositoryError;
use crate::features::feedback::repositories::errors::delete_feedback_repository_error::DeleteFeedbackRepositoryError;
use crate::features::feedback::services::errors::delete_feedback_service_error::DeleteFeedbackServiceError;
use crate::features::feedback::services::feedback_deletion_service::FeedbackDeletionService;
use crate::shared::jwt::Claims;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

pub async fn delete_feedback(
    State(state): State<ServiceAppState<Arc<dyn FeedbackDeletionService>>>,
    _claims: Claims,
    Json(request): Json<DeleteFeedbackRequest>,
) -> Result<(), StatusCode> {
    tracing::debug!(?request, "Incoming DELETE request to /feedback");
    state
        .service
        .delete_feedback(FeedbackDeletionMode::Multiple(request.into()))
        .await
        .map_err(move |err| match err {
            DeleteFeedbackServiceError::RepositorySingleDeletionError(repository_error) => {
                tracing::error!(?repository_error, "Unexpected error deleting feedback");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            DeleteFeedbackServiceError::RepositoryMultiDeletionError(
                DeleteFeedbackRepositoryError::FeedbackNotFound,
            ) => StatusCode::NOT_FOUND,
            DeleteFeedbackServiceError::RepositoryMultiDeletionError(
                DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
            ) => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::feedback::services::feedback_deletion_service::MockFeedbackDeletionService;
    use masjid_app_api_library::features::feedback::models::feedback_type::FeedbackType;
    use masjid_app_api_library::features::feedback::models::suggestion_type::SuggestionType;

    #[tokio::test]
    async fn test_delete_feedback() {
        struct TestCase {
            description: &'static str,
            request: DeleteFeedbackRequest,
            mock_service_result: Result<(), DeleteFeedbackServiceError>,
            expected_result: Result<(), StatusCode>,
        }
        let request = DeleteFeedbackRequest {
            feedback_type: FeedbackType::Suggestion(SuggestionType::Management),
            date_of_feedback: None,
        };
        let test_cases = [
            TestCase {
                description: "When deleting non-existent feedback records, I should get a NOT_FOUND response",
                request: request.clone(),
                mock_service_result: Err(DeleteFeedbackServiceError::RepositoryMultiDeletionError(
                    DeleteFeedbackRepositoryError::FeedbackNotFound,
                )),
                expected_result: Err(StatusCode::NOT_FOUND),
            },
            TestCase {
                description: "When deletion of feedback fails, I should get an INTERNAL_SERVER_ERROR response",
                request,
                mock_service_result: Err(DeleteFeedbackServiceError::RepositoryMultiDeletionError(
                    DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
                )),
                expected_result: Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
        ];

        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockFeedbackDeletionService::new();

            mock_service
                .expect_delete_feedback()
                .return_once(move |_| test_case.mock_service_result);
            let arc_service: Arc<dyn FeedbackDeletionService> = Arc::new(mock_service);
            let app_state = ServiceAppState {
                service: arc_service,
            };
            let actual_result =
                delete_feedback(State(app_state), Claims::default(), Json(test_case.request)).await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
