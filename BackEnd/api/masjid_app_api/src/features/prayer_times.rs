use crate::shared::app_state::{AppState, DbType};
use crate::shared::repository_manager::{InMemoryRepository, MySqlRepository, RepositoryType};
use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use mockall::predicate::*;
use mockall::*;
use serde::Deserialize;
use sqlx::mysql::MySqlRow;
use sqlx::{Error, Row};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub enum GetPrayerTimesError {
    PrayerTimesNotFound,
    UnableToGetPrayerTimes,
}
#[derive(Deserialize, Clone)]
pub struct UpdatePrayerTimesRequest {
    pub prayer_times_data: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum UpdatePrayerTimesError {
    UnableToUpdatePrayerTimes,
}

#[automock]
#[async_trait]
pub trait PrayerTimesRepository: Send + Sync {
    async fn get_prayer_times(&self) -> Result<Vec<u8>, GetPrayerTimesError>;
    async fn update_prayer_times(
        &self,
        prayer_times_data: &[u8],
    ) -> Result<(), UpdatePrayerTimesError>;
}

pub async fn new_prayer_times_repository(db_type: DbType) -> Arc<dyn PrayerTimesRepository> {
    match db_type {
        DbType::InMemory => Arc::new(InMemoryRepository::new(RepositoryType::PrayerTimes).await),
        DbType::MySql => Arc::new(MySqlRepository::new(RepositoryType::PrayerTimes).await),
    }
}
#[async_trait]
impl PrayerTimesRepository for InMemoryRepository {
    async fn get_prayer_times(&self) -> Result<Vec<u8>, GetPrayerTimesError> {
        todo!()
    }

    async fn update_prayer_times(
        &self,
        prayer_times_data: &[u8],
    ) -> Result<(), UpdatePrayerTimesError> {
        todo!()
    }
}
#[async_trait]
impl PrayerTimesRepository for MySqlRepository {
    async fn get_prayer_times(&self) -> Result<Vec<u8>, GetPrayerTimesError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL get_prayer_times_file();")
            .fetch_one(&*db_connection)
            .await
            .map(|row: MySqlRow| row.get(0));
        match query_response {
            Ok(prayer_times) => Ok(prayer_times),
            Err(Error::RowNotFound) => Err(GetPrayerTimesError::PrayerTimesNotFound),
            Err(e) => Err(GetPrayerTimesError::UnableToGetPrayerTimes),
        }
    }

    async fn update_prayer_times(
        &self,
        prayer_times_data: &[u8],
    ) -> Result<(), UpdatePrayerTimesError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL upsert_prayer_times_file(?);")
            .bind(prayer_times_data)
            .execute(&*db_connection)
            .await;
        match query_response {
            Ok(_) => Ok(()),
            Err(_) => Err(UpdatePrayerTimesError::UnableToUpdatePrayerTimes),
        }
    }
}

pub async fn get_prayer_times(
    State(state): State<AppState<Arc<dyn PrayerTimesRepository>>>,
) -> Response {
    let mut get_prayer_times_result = state
        .repository_map
        .get(&DbType::InMemory)
        .unwrap()
        .get_prayer_times()
        .await;
    if get_prayer_times_result.is_err() {
        get_prayer_times_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .get_prayer_times()
            .await;
    }
    match get_prayer_times_result {
        Ok(prayer_times) => (StatusCode::OK, prayer_times).into_response(),
        Err(GetPrayerTimesError::PrayerTimesNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(GetPrayerTimesError::UnableToGetPrayerTimes) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_prayer_times(
    State(state): State<AppState<Arc<dyn PrayerTimesRepository>>>,
    Json(request): Json<UpdatePrayerTimesRequest>,
) -> Response {
    if request.prayer_times_data.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            "Prayer times data cannot be empty.",
        )
            .into_response();
    }
    let mut update_prayer_times_result = state
        .repository_map
        .get(&DbType::InMemory)
        .unwrap()
        .update_prayer_times(&request.prayer_times_data)
        .await;
    if update_prayer_times_result.is_err() {
        update_prayer_times_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .update_prayer_times(&request.prayer_times_data)
            .await;
    }
    match update_prayer_times_result {
        Ok(()) => StatusCode::OK.into_response(),
        Err(UpdatePrayerTimesError::UnableToUpdatePrayerTimes) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[tokio::test]
    async fn test_get_prayer_times() {
        #[derive(Clone)]
        struct TestCase {
            cached_prayer_times_data: Result<Vec<u8>, GetPrayerTimesError>,
            prayer_times_data: Result<Vec<u8>, GetPrayerTimesError>,
            expected_response_code: StatusCode,
        }

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
                cached_prayer_times_data: Ok(vec![1, 2, 3, 4, 5]),
                prayer_times_data: Ok(vec![1, 2, 3, 4, 5]),
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

            let actual_response = get_prayer_times(State(app_state)).await;

            // Assert response matches expected status code
            assert_eq!(case.expected_response_code, actual_response.status());
        }
    }

    #[tokio::test]
    async fn test_update_prayer_times() {
        #[derive(Clone)]
        struct TestCase {
            cached_prayer_times_data: Vec<u8>,
            prayer_times_data: Vec<u8>,
            expected_in_memory_db_response: Option<Result<(), UpdatePrayerTimesError>>,
            expected_db_response: Option<Result<(), UpdatePrayerTimesError>>,
            expected_api_response_code: StatusCode,
        }
        let test_cases = vec![
            TestCase {
                cached_prayer_times_data: vec![],
                prayer_times_data: vec![],
                expected_api_response_code: StatusCode::BAD_REQUEST,
                expected_in_memory_db_response: None,
                expected_db_response: None,
            },
            TestCase {
                cached_prayer_times_data: vec![1, 2, 3, 4, 5],
                prayer_times_data: vec![1, 2, 3, 4, 5],
                expected_api_response_code: StatusCode::INTERNAL_SERVER_ERROR,
                expected_in_memory_db_response: Some(Err(
                    UpdatePrayerTimesError::UnableToUpdatePrayerTimes,
                )),
                expected_db_response: Some(Err(UpdatePrayerTimesError::UnableToUpdatePrayerTimes)),
            },
            TestCase {
                cached_prayer_times_data: vec![1, 2, 3, 4, 5],
                prayer_times_data: vec![1, 2, 3, 4, 5],
                expected_api_response_code: StatusCode::OK,
                expected_in_memory_db_response: Some(Ok(())),
                expected_db_response: Some(Ok(())),
            },
        ];
        for test_case in test_cases {
            let mut mock_prayer_times_in_memory_repository = MockPrayerTimesRepository::new();
            let mut mock_prayer_times_repository = MockPrayerTimesRepository::new();
            if let Some(expected_in_memory_db_response) = test_case.expected_in_memory_db_response {
                mock_prayer_times_in_memory_repository
                    .expect_update_prayer_times()
                    .returning(move |data| expected_in_memory_db_response.clone());
            }
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_prayer_times_repository
                    .expect_update_prayer_times()
                    .returning(move |data| expected_db_response.clone());
            }
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
            let actual_response = update_prayer_times(
                State(app_state),
                Json::from(UpdatePrayerTimesRequest {
                    prayer_times_data: test_case.prayer_times_data.clone(),
                }),
            );
        }
    }
}
