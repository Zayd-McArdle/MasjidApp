use crate::features::donation::models::SendDonationRequest;
use crate::features::donation::services::DonationPublicService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

pub async fn send_donation(
    State(app_state): State<ServiceAppState<Arc<dyn DonationPublicService>>>,
    Json(request): Json<SendDonationRequest>,
) -> Response {
    if request.validate().is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    StatusCode::CREATED.into_response()
}
