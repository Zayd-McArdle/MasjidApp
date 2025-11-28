use axum::extract::State;
use axum::response::Response;
use masjid_app_api_library::features::events::endpoints::get_events_common;
use masjid_app_api_library::features::events::repositories::EventsRepository;
use masjid_app_api_library::shared::types::app_state::AppState;
use std::sync::Arc;

pub async fn get_events(State(state): State<AppState<Arc<dyn EventsRepository>>>) -> Response {
    get_events_common(State(state)).await
}
