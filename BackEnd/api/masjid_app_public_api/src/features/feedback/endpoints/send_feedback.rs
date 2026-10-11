use crate::features::feedback::errors::insert_feedback_repository_error::InsertFeedbackRepositoryError;
use crate::features::feedback::errors::send_feedback_error::SendFeedbackError;
use crate::features::feedback::models::send_feedback_request::SendFeedbackRequest;
use crate::features::feedback::services::feedback_publishing_service::FeedbackPublishingService;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

pub async fn send_feedback(
    State(state): State<ServiceAppState<Arc<dyn FeedbackPublishingService>>>,
    Json(request): Json<SendFeedbackRequest>,
) -> Result<(), StatusCode> {
    tracing::debug!(?request, "Incoming POST request for /feedback");
    request
        .validate()
        .map_err(move |_| StatusCode::BAD_REQUEST)?;
    state
        .service
        .send_feedback(request.into())
        .await
        .map_err(move |err| match err {
            SendFeedbackError::RepositoryError(
                InsertFeedbackRepositoryError::UnableToInsertFeedback,
            ) => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::feedback::services::feedback_publishing_service::MockFeedbackPublishingService;
    use masjid_app_api_library::features::feedback::models::concern_type::ConcernType;
    use masjid_app_api_library::features::feedback::models::feedback_type::FeedbackType;
    use masjid_app_api_library::features::feedback::models::suggestion_type::SuggestionType;

    #[tokio::test]
    async fn test_send_feedback() {
        struct TestCase {
            description: &'static str,
            request: SendFeedbackRequest,
            mock_service_result: Option<Result<(), SendFeedbackError>>,
            expected_result: Result<(), StatusCode>,
        }
        let valid_request = SendFeedbackRequest {
            title: "new islamic talk".to_string(),
            feedback_type: FeedbackType::Suggestion(SuggestionType::Proposal),
            details: "I am interested to see a talk on aqeedah and fiqh".to_string(),
            images: None,
        };
        let test_cases = [
            TestCase {
                description: "When an invalid request is made, I should get a BAD_REQUEST",
                request: SendFeedbackRequest {
                    title: "".to_string(),
                    feedback_type: FeedbackType::Concern(ConcernType::Other(String::new())),
                    details: "".to_string(),
                    images: None,
                },
                mock_service_result: None,
                expected_result: Err(StatusCode::BAD_REQUEST),
            },
            TestCase {
                description: "When sending feedback fails, I should get an INTERNAL_SERVER_ERROR",
                request: valid_request.clone(),
                mock_service_result: Some(Err(SendFeedbackError::RepositoryError(
                    InsertFeedbackRepositoryError::UnableToInsertFeedback,
                ))),
                expected_result: Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            TestCase {
                description: "When sending feedback succeeds, I should get no error",
                request: valid_request,
                mock_service_result: Some(Ok(())),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockFeedbackPublishingService::new();
            if let Some(mock_service_result) = test_case.mock_service_result {
                mock_service
                    .expect_send_feedback()
                    .return_once(move |_| mock_service_result);
            }

            let app_state = ServiceAppState::<Arc<dyn FeedbackPublishingService>> {
                service: Arc::new(mock_service),
            };

            let actual_result = send_feedback(State(app_state), Json(test_case.request)).await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
