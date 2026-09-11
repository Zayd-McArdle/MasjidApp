use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use masjid_app_api_library::features::events::endpoints::get_events::get_events_common;
use masjid_app_api_library::features::events::models::event_dto::EventDTO;
use masjid_app_api_library::features::events::services::event_retrieval_service::EventRetrievalService;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

use crate::shared::jwt::Claims;

pub async fn get_events(
    State(state): State<ServiceAppState<Arc<dyn EventRetrievalService>>>,
    _claims: Claims,
) -> Result<Json<Vec<EventDTO>>, StatusCode> {
    get_events_common(State(state)).await
}
