use crate::features::prayer_times::services::errors::check_for_updated_prayer_times_error::CheckForUpdatedPrayerTimesError;
use crate::features::prayer_times::services::prayer_times_update_checking_service::PrayerTimesUpdateCheckingService;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::features::prayer_times::endpoints::utils::build_prayer_times_response;
use masjid_app_api_library::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

pub async fn get_updated_prayer_times(
    State(state): State<ServiceAppState<Arc<dyn PrayerTimesUpdateCheckingService>>>,
    hash: Path<String>,
) -> Response {
    if hash.len() != 64 {
        return (
            StatusCode::BAD_REQUEST,
            format!("Malformed hash: {}", hash.0),
        )
            .into_response();
    }

    match state.service.check_for_updated_prayer_times(&hash).await {
        Ok(prayer_times) => build_prayer_times_response(prayer_times, Some(&hash)),
        Err(CheckForUpdatedPrayerTimesError::RepositoryError(
            GetPrayerTimesRepositoryError::PrayerTimesNotFound,
        )) => StatusCode::NOT_FOUND.into_response(),
        Err(CheckForUpdatedPrayerTimesError::RepositoryError(
            GetPrayerTimesRepositoryError::UnableToGetPrayerTimes,
        )) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
