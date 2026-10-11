use crate::features::feedback::models::filters::get_feedback_filter::GetFeedbackFilter;
use crate::features::feedback::repositories::errors::get_feedback_repository_error::GetFeedbackRepositoryError;
use crate::features::feedback::services::errors::get_feedback_service_error::GetFeedbackServiceError;
use crate::features::feedback::services::feedback_retrieval_service::FeedbackRetrievalService;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use masjid_app_api_library::features::feedback::models::feedback_dto::FeedbackDTO;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

pub mod get_concerns;
pub mod get_suggestions;

#[inline]
async fn get_feedback(
    State(state): State<ServiceAppState<Arc<dyn FeedbackRetrievalService>>>,
    Json(request): Json<impl Into<GetFeedbackFilter>>,
) -> Result<Json<Vec<FeedbackDTO>>, StatusCode> {
    state
        .service
        .retrieve_feedback(request.into())
        .await
        .map(Json::from)
        .map_err(move |err| match err {
            GetFeedbackServiceError::RepositoryError(GetFeedbackRepositoryError::NotFound) => {
                StatusCode::NOT_FOUND
            }
            GetFeedbackServiceError::RepositoryError(
                GetFeedbackRepositoryError::UnableToRetrieveFeedback,
            ) => StatusCode::INTERNAL_SERVER_ERROR,
        })
}
