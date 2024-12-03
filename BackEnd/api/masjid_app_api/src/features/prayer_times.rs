use crate::shared::app_state::InnerAppState;
use crate::shared::repository_manager::{ConnectionString, MainRepository, RepositoryMode};
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
enum GetPrayerTimesError {
    PrayerTimesNotFound,
    UnableToGetPrayerTimes,
}
#[derive(Deserialize)]
pub struct UpdatePrayerTimesRequest {
    pub prayer_times_data: Vec<u8>
}
enum UpdatePrayerTimesError {
    UnableToUpdatePrayerTimes,
}

#[automock]
#[async_trait]
pub trait PrayerTimesRepository {
    async fn get_prayer_times(&self) -> Result<Vec<u8>, GetPrayerTimesError>;
    async fn update_prayer_times(&self, prayer_times_data: Vec<u8>) -> Result<(), UpdatePrayerTimesError>;
}

pub async fn new_prayer_times_repository(mode: RepositoryMode) -> Arc<dyn PrayerTimesRepository> {
    Arc::new(MainRepository::new(ConnectionString::PrayerTimesConnection).await)
}
#[async_trait]
impl PrayerTimesRepository for MainRepository {
    async fn get_prayer_times(&self) -> Result<Vec<u8>, GetPrayerTimesError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query_scalar::<_, Vec<u8>>("CALL get_prayer_times();")
            .fetch_one(&*db_connection).await;
        match query_response {
            Ok(prayer_times) => Ok(prayer_times),
            Err(Error::RowNotFound) => Err(GetPrayerTimesError::PrayerTimesNotFound),
            Err(_) => Err(GetPrayerTimesError::UnableToGetPrayerTimes),
        }
    }

    async fn update_prayer_times(&self, prayer_times_data: Vec<u8>) -> Result<(), UpdatePrayerTimesError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL update_prayer_times(?);")
            .bind(prayer_times_data)
            .execute(&*db_connection).await;
        match query_response {
            Ok(_) => Ok(()),
            Err(_) => Err(UpdatePrayerTimesError::UnableToUpdatePrayerTimes)
        }
    }
}



pub async fn get_prayer_times(State(state): State<InnerAppState<Arc<dyn PrayerTimesRepository>>>) -> Response {
    
    match state.repositories[0].get_prayer_times().await {
        Ok(prayer_times) => (StatusCode::OK, prayer_times).into_response(),
        Err(GetPrayerTimesError::PrayerTimesNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(GetPrayerTimesError::UnableToGetPrayerTimes) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_prayer_times(Json(request): Json<UpdatePrayerTimesRequest>, State(state): State<InnerAppState<Arc<dyn PrayerTimesRepository>>>) -> Response {
    if request.prayer_times_data.is_empty() {
        return (StatusCode::BAD_REQUEST, "Prayer times data cannot be empty.").into_response()
    }
    match state.repositories[0].update_prayer_times(request.prayer_times_data).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(UpdatePrayerTimesError::UnableToUpdatePrayerTimes) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_prayer_times() {
        struct TestCase {
            prayer_times_data: Result<Vec<u8>, GetPrayerTimesError>,
            expected_response_code: StatusCode,
        }
        let test_cases = vec![
            TestCase {
                prayer_times_data: Err(GetPrayerTimesError::PrayerTimesNotFound),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                prayer_times_data: Ok(vec![1, 2, 3, 4, 5] ),
                expected_response_code: StatusCode::OK,
            }
        ];
        for test_case in test_cases {
            let mut mock_prayer_times_repository = MockPrayerTimesRepository::new();
            let app_state:InnerAppState<Arc<dyn PrayerTimesRepository>> = InnerAppState{repositories: vec![Arc::new(mock_prayer_times_repository)]};
            mock_prayer_times_repository.expect_get_prayer_times().return_once(|| test_case.prayer_times_data);        

            let actual_response = get_prayer_times(State(app_state)).await;
            assert_eq!(test_case.expected_response_code, actual_response.status())
        }
    }
}