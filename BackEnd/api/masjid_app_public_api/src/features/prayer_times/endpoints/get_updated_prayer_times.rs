use crate::features::prayer_times::services::errors::check_for_updated_prayer_times_error::CheckForUpdatedPrayerTimesError;
use crate::features::prayer_times::services::prayer_times_update_checking_service::PrayerTimesUpdateCheckingService;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use masjid_app_api_library::features::prayer_times::endpoints::utils::build_prayer_times_response;
use masjid_app_api_library::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

pub async fn get_updated_prayer_times(
    State(state): State<ServiceAppState<Arc<dyn PrayerTimesUpdateCheckingService>>>,
    hash: Path<String>,
) -> Result<Response<Body>, StatusCode> {
    if hash.len() != 64 {
        tracing::warn!(hash = ?hash, "The hash of updated prayer times was invalid");
        return Err(StatusCode::BAD_REQUEST);
    }
    let prayer_times = state
        .service
        .check_for_updated_prayer_times(&hash)
        .await
        .map_err(move |err| match err {
            CheckForUpdatedPrayerTimesError::RepositoryError(
                GetPrayerTimesRepositoryError::PrayerTimesNotFound,
            ) => StatusCode::NOT_FOUND,
            CheckForUpdatedPrayerTimesError::RepositoryError(
                GetPrayerTimesRepositoryError::UnableToGetPrayerTimes,
            ) => StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    build_prayer_times_response(prayer_times, Some(&hash))
}
