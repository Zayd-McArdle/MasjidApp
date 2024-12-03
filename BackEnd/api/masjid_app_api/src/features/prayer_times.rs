use std::sync::Arc;
use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use validator::Validate;
use crate::features::announcements::announcements::{AnnouncementDTO, GetAnnouncementsError};
use crate::shared::app_state::AppState;
use crate::shared::repository_manager::{ConnectionString, Repository};

#[derive(Serialize, FromRow)]
pub struct PrayerTimesDTO {
    pub prayer_times_data: Vec<i8>
}
enum GetPrayerTimesError {
    PrayerTimesNotFound,
    UnableToGetPrayerTimes,
}
#[derive(Deserialize)]
pub struct UpdatePrayerTimesRequest {
    pub prayer_times_data: Vec<i8>
}
enum UpdatePrayerTimesError {
    UnableToUpdatePrayerTimes,
}

#[async_trait]
pub trait PrayerTimesRepository {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError>;
    async fn update_prayer_times(&self, prayer_times: PrayerTimesDTO) -> Result<(), UpdatePrayerTimesError>;
}

pub async fn new_prayer_times_repository() -> Arc<dyn PrayerTimesRepository> {
    Arc::new(Repository::new(ConnectionString::PrayerTimesConnection))
}

impl PrayerTimesRepository for Repository {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError> {
        let query_response: Result<PrayerTimesDTO, Error> = sqlx::query_as("CALL get_prayer_times();").fetch_all(&self.db_connection).await;
        match query_response {
            Ok(prayer_times) => Ok(prayer_times),
            Err(_RowNotFound) => Err(GetPrayerTimesError::PrayerTimesNotFound),
            Err(_) => Err(GetPrayerTimesError::UnableToGetPrayerTimes)
        }
    }

    async fn update_prayer_times(&self, prayer_times: PrayerTimesDTO) -> Result<(), UpdatePrayerTimesError> {
        todo!()
    }
}



pub async fn get_prayer_times(State(state): State<AppState>) -> Response {
    match state.prayer_times_repository.get_prayer_times().await {
        Ok(prayer_times) => (StatusCode::OK, prayer_times.prayer_times_data).into_response(),
        Err(GetPrayerTimesError::PrayerTimesNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(GetPrayerTimesError::UnableToGetPrayerTimes) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_prayer_times(Json(request): Json<UpdatePrayerTimesRequest>, State(state): State<AppState>) -> Response {
    if request.prayer_times_data.is_empty() {
        (StatusCode::BAD_REQUEST, "Prayer times data cannot be empty.").into_response()
    }
    match state.prayer_times_repository.update_prayer_times() {  }
}