use crate::shared::app_state::AppState;
use crate::shared::repository_manager::{ConnectionString, Repository};
use mockall::*;
use mockall::predicate::*;
use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use std::sync::Arc;

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

#[automock]
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
        let query_response: Result<PrayerTimesDTO, Error> = sqlx::query_as::<_, PrayerTimesDTO>("CALL get_prayer_times();")
            .fetch_one(&self.db_connection).await;
        match query_response {
            Ok(prayer_times) => Ok(prayer_times),
            Err(_RowNotFound) => Err(GetPrayerTimesError::PrayerTimesNotFound),
        }
    }

    async fn update_prayer_times(&self, prayer_times: PrayerTimesDTO) -> Result<(), UpdatePrayerTimesError> {
        let query_response = sqlx::query("CALL update_prayer_times(?);")
            .bind(prayer_times.prayer_times_data)
            .execute(&self.db_connection).await;
        match query_response {
            Ok(_) => Ok(()),
            Err(_) => Err(UpdatePrayerTimesError::UnableToUpdatePrayerTimes)
        }
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
        return (StatusCode::BAD_REQUEST, "Prayer times data cannot be empty.").into_response()
    }
    match state.prayer_times_repository.update_prayer_times(PrayerTimesDTO{prayer_times_data: request.prayer_times_data}) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(UpdatePrayerTimesError::UnableToUpdatePrayerTimes) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_prayer_times() {
        struct TestCase {
            prayer_times_data: Result<PrayerTimesDTO, GetPrayerTimesError>,
            expected_response_code: StatusCode,
        }
        let test_cases = vec![
            TestCase {
                prayer_times_data: Err(GetPrayerTimesError::PrayerTimesNotFound),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                prayer_times_data: Ok(PrayerTimesDTO{ prayer_times_data: vec![1, 2, 3, 4, 5] }),
                expected_response_code: StatusCode::OK,
            }
        ];
        for test_case in test_cases {
            let mut app_state = AppState::default();
            let mut mock_prayer_times_repository = Arc::new(MockPrayerTimesRepository::new());
            mock_prayer_times_repository.expect_get_prayer_times().return_once(|| test_case.prayer_times_data);
            app_state.prayer_times_repository = Arc::new(MockPrayerTimesRepository::new());

            let actual_response = get_prayer_times(State(app_state));
            assert_eq!(test_case.expected_response_code, actual_response.status())
        }
    }
}