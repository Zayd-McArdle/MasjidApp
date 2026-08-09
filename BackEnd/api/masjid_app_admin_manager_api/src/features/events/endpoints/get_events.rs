use axum::extract::State;
use axum::response::Response;
use masjid_app_api_library::features::events::endpoints::get_events::get_events_common;
use masjid_app_api_library::features::events::services::event_retrieval_service::EventRetrievalService;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

pub async fn get_events(
    State(state): State<ServiceAppState<Arc<dyn EventRetrievalService>>>,
) -> Response {
    get_events_common(State(state)).await
}
