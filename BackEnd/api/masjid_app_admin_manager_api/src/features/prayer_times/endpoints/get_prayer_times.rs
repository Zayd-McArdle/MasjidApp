use crate::shared::jwt::Claims;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use masjid_app_api_library::features::prayer_times::endpoints::get_prayer_times::get_prayer_times_common;
use masjid_app_api_library::features::prayer_times::services::prayer_times_retrieval_service::PrayerTimesRetrievalService;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

#[inline]
pub async fn get_prayer_times(
    State(state): State<ServiceAppState<Arc<dyn PrayerTimesRetrievalService>>>,
    _claims: Claims,
) -> Result<Response<Body>, StatusCode> {
    get_prayer_times_common(State(state)).await
}
