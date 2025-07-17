use std::sync::Arc;
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use mockall::automock;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Row};
use sqlx::mysql::MySqlRow;
use crate::shared::app_state::{AppState, DbType};
use crate::shared::repository_manager::{InMemoryRepository, MySqlRepository, RepositoryType};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PrayerTimesDTO {
    pub data: Option<Vec<u8>>,
    pub hash: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GetPrayerTimesError {
    PrayerTimesNotFound,
    UnableToGetPrayerTimes,
}
pub fn build_prayer_times_response(prayer_times: PrayerTimesDTO, hash: Option<&str>) -> Response {
    if let Some(hash_value) = hash {
        if prayer_times.hash == hash_value.to_owned() {
            return StatusCode::CONFLICT.into_response();
        }
    }
    if let Some(data) = prayer_times.data {
        // Create response_body_result with hash in a custom header
        let response_body_result = Response::builder()
            .status(StatusCode::OK)
            .header("X-File-Hash", prayer_times.hash)
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .body(Body::from(data));
        return match response_body_result {
            Ok(response) => response,
            Err(err) => {
                tracing::error!("unable to build response: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        };
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
#[automock]
#[async_trait]
pub trait PrayerTimesRepository: Send + Sync {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError>;
}

#[async_trait]
impl PrayerTimesRepository for MySqlRepository {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL get_prayer_times();")
            .fetch_one(&*db_connection)
            .await
            .map(|row: MySqlRow| PrayerTimesDTO {
                data: row.get(0),
                hash: row.get(1),
            });

        match query_response {
            Ok(prayer_times) => Ok(prayer_times),
            Err(Error::RowNotFound) => {
                Err(GetPrayerTimesError::PrayerTimesNotFound)
            }
            Err(err) => {
                tracing::error!("unable to get prayer times from the database: {}", err);
                Err(GetPrayerTimesError::UnableToGetPrayerTimes)
            }
        }
    }
}
pub async fn get_prayer_times_common(
    State(state): State<AppState<Arc<dyn PrayerTimesRepository>>>,
) -> Response {
    let mut get_prayer_times_result: Result<PrayerTimesDTO, GetPrayerTimesError> = Err(GetPrayerTimesError::UnableToGetPrayerTimes);

    if let Some(prayer_times_in_memory_repository) = state.repository_map.get(&DbType::InMemory) {
        get_prayer_times_result = prayer_times_in_memory_repository.get_prayer_times().await;
    }

    if get_prayer_times_result.is_err() {
        get_prayer_times_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .get_prayer_times()
            .await;
    }
    match get_prayer_times_result {
        Ok(prayer_times) => build_prayer_times_response(prayer_times, None),
        Err(GetPrayerTimesError::PrayerTimesNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(GetPrayerTimesError::UnableToGetPrayerTimes) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

mod test {
    use super::*;
    use crate::shared::app_state::AppState;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_prayer_times() {
        #[derive(Clone)]
        struct TestCase {
            cached_prayer_times_data: Result<PrayerTimesDTO, GetPrayerTimesError>,
            prayer_times_data: Result<PrayerTimesDTO, GetPrayerTimesError>,
            expected_response_code: StatusCode,
        }
        let valid_prayer_times_data = Ok(PrayerTimesDTO {
            data: Some(vec![1, 2, 3, 4, 5]),
            hash: "5a4e9c5d6b8a2f3e1c0b9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d7".to_owned(),
        });
        let test_cases = vec![
            TestCase {
                cached_prayer_times_data: Err(GetPrayerTimesError::PrayerTimesNotFound),
                prayer_times_data: Err(GetPrayerTimesError::PrayerTimesNotFound),
                expected_response_code: StatusCode::NOT_FOUND,
            },
            TestCase {
                cached_prayer_times_data: Err(GetPrayerTimesError::UnableToGetPrayerTimes),
                prayer_times_data: Err(GetPrayerTimesError::UnableToGetPrayerTimes),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                cached_prayer_times_data: valid_prayer_times_data.clone(),
                prayer_times_data: valid_prayer_times_data,
                expected_response_code: StatusCode::OK,
            },
        ];

        for case in test_cases {
            let mut mock_prayer_times_in_memory_repository = MockPrayerTimesRepository::new();
            let mut mock_prayer_times_repository = MockPrayerTimesRepository::new();

            mock_prayer_times_in_memory_repository
                .expect_get_prayer_times()
                .returning(move || case.cached_prayer_times_data.clone());
            mock_prayer_times_repository
                .expect_get_prayer_times()
                .returning(move || case.prayer_times_data.clone());
            let arc_in_memory_repository: Arc<dyn PrayerTimesRepository> =
                Arc::new(mock_prayer_times_in_memory_repository);
            let arc_repository: Arc<dyn PrayerTimesRepository> =
                Arc::new(mock_prayer_times_repository);
            let app_state: AppState<Arc<dyn PrayerTimesRepository>> = AppState {
                repository_map: HashMap::from([
                    (DbType::InMemory, arc_in_memory_repository),
                    (DbType::MySql, arc_repository),
                ]),
            };

            let actual_response = get_prayer_times_common(State(app_state)).await;

            // Assert response matches expected status code
            assert_eq!(case.expected_response_code, actual_response.status());
        }
    }

}