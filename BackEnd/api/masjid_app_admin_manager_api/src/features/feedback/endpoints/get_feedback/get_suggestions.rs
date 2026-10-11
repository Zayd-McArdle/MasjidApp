use crate::features::feedback::endpoints::get_feedback::get_feedback;
use crate::features::feedback::models::requests::get_suggestions_request::GetSuggestionsRequest;
use crate::features::feedback::services::feedback_retrieval_service::FeedbackRetrievalService;
use crate::shared::jwt::Claims;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use masjid_app_api_library::features::feedback::models::feedback_dto::FeedbackDTO;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
pub async fn get_suggestions(
    State(state): State<ServiceAppState<Arc<dyn FeedbackRetrievalService>>>,
    _claims: Claims,
    Json(request): Json<GetSuggestionsRequest>,
) -> Result<Json<Vec<FeedbackDTO>>, StatusCode> {
    get_feedback(State(state), Json(request)).await
}
