use crate::shared::app_state::{AppState, DbType};
use crate::shared::jwt::Claims;
use crate::shared::repository_manager::{InMemoryRepository, MySqlRepository, RepositoryType};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use mockall::predicate::*;
use mockall::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::mysql::MySqlRow;
use sqlx::{Error, Row};
use std::sync::Arc;
use validator::Validate;

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

#[derive(Deserialize, Clone, Validate)]
pub struct UpdatePrayerTimesRequest {
    #[serde(rename = "prayerTimesData")]
    pub prayer_times_data: Vec<u8>,
    #[validate(length(equal = 64))]
    pub hash: String,
}
#[derive(Clone, Debug, PartialEq)]
pub enum UpdatePrayerTimesError {
    UnableToUpdatePrayerTimes,
}

fn build_prayer_times_response(prayer_times: PrayerTimesDTO, hash: Option<&str>) -> Response {
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
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

#[automock]
#[async_trait]
pub trait PrayerTimesRepository: Send + Sync {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError>;
    async fn get_updated_prayer_times(
        &self,
        hash: &str,
    ) -> Result<PrayerTimesDTO, GetPrayerTimesError>;
    async fn update_prayer_times(
        &self,
        prayer_times_data: PrayerTimesDTO,
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
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError> {
        todo!()
    }

    async fn get_updated_prayer_times(
        &self,
        hash: &str,
    ) -> Result<PrayerTimesDTO, GetPrayerTimesError> {
        todo!()
    }

    async fn update_prayer_times(
        &self,
        prayer_times_data: PrayerTimesDTO,
    ) -> Result<(), UpdatePrayerTimesError> {
        todo!()
    }
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
            Err(Error::RowNotFound) => Err(GetPrayerTimesError::PrayerTimesNotFound),
            Err(e) => Err(GetPrayerTimesError::UnableToGetPrayerTimes),
        }
    }

    async fn get_updated_prayer_times(
        &self,
        hash: &str,
    ) -> Result<PrayerTimesDTO, GetPrayerTimesError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL get_updated_prayer_times(?);")
            .bind(hash)
            .fetch_one(&*db_connection)
            .await
            .map(|row: MySqlRow| {
                if row.len() == 1 {
                    return PrayerTimesDTO {
                        data: None,
                        hash: row.get(0),
                    };
                }
                return PrayerTimesDTO {
                    data: row.get(0),
                    hash: row.get(1),
                };
            });
        match query_response {
            Ok(prayer_times) => Ok(prayer_times),
            Err(Error::RowNotFound) => Err(GetPrayerTimesError::PrayerTimesNotFound),
            Err(e) => {
                println!("{}", e);
                Err(GetPrayerTimesError::UnableToGetPrayerTimes)
            }
        }
    }

    async fn update_prayer_times(
        &self,
        prayer_times_data: PrayerTimesDTO,
    ) -> Result<(), UpdatePrayerTimesError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL upsert_prayer_times(?, ?);")
            .bind(prayer_times_data.data)
            .bind(prayer_times_data.hash)
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
        Ok(prayer_times) => build_prayer_times_response(prayer_times, None),
        Err(GetPrayerTimesError::PrayerTimesNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(GetPrayerTimesError::UnableToGetPrayerTimes) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_updated_prayer_times(
    State(state): State<AppState<Arc<dyn PrayerTimesRepository>>>,
    hash: Path<String>,
) -> Response {
    if hash.len() != 64 {
        return (
            StatusCode::BAD_REQUEST,
            format!("Malformed hash: {}", hash.0),
        )
            .into_response();
    }
    let mut get_hash_result = state
        .repository_map
        .get(&DbType::InMemory)
        .unwrap()
        .get_updated_prayer_times(&hash)
        .await;
    if get_hash_result.is_err() {
        get_hash_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .get_updated_prayer_times(&hash)
            .await;
    }
    match get_hash_result {
        Ok(prayer_times) => build_prayer_times_response(prayer_times, Some(&hash)),
        Err(GetPrayerTimesError::PrayerTimesNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(GetPrayerTimesError::UnableToGetPrayerTimes) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_prayer_times(
    State(state): State<AppState<Arc<dyn PrayerTimesRepository>>>,
    claims: Claims,
    Json(request): Json<UpdatePrayerTimesRequest>,
) -> Response {
    if request.validate().is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let hashed_prayer_times = format!("{:x}", Sha256::digest(&request.prayer_times_data));
    if request.hash != hashed_prayer_times {
        return (
            StatusCode::BAD_REQUEST,
            "Verification of prayer times failed",
        )
            .into_response();
    }
    let prayer_times = PrayerTimesDTO {
        data: Some(request.prayer_times_data),
        hash: request.hash,
    };
    let mut update_prayer_times_result = state
        .repository_map
        .get(&DbType::InMemory)
        .unwrap()
        .update_prayer_times(prayer_times.clone())
        .await;
    if update_prayer_times_result.is_err() {
        update_prayer_times_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .update_prayer_times(prayer_times)
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
            claims: Claims,
        }
        let test_cases = vec![
            TestCase {
                cached_prayer_times_data: vec![],
                prayer_times_data: vec![],
                expected_api_response_code: StatusCode::BAD_REQUEST,
                expected_in_memory_db_response: None,
                expected_db_response: None,
                claims: Default::default(),
            },
            TestCase {
                cached_prayer_times_data: vec![1, 2, 3, 4, 5],
                prayer_times_data: vec![1, 2, 3, 4, 5],
                expected_api_response_code: StatusCode::INTERNAL_SERVER_ERROR,
                expected_in_memory_db_response: Some(Err(
                    UpdatePrayerTimesError::UnableToUpdatePrayerTimes,
                )),
                expected_db_response: Some(Err(UpdatePrayerTimesError::UnableToUpdatePrayerTimes)),
                claims: Default::default(),
            },
            TestCase {
                cached_prayer_times_data: vec![1, 2, 3, 4, 5],
                prayer_times_data: vec![1, 2, 3, 4, 5],
                expected_api_response_code: StatusCode::OK,
                expected_in_memory_db_response: Some(Ok(())),
                expected_db_response: Some(Ok(())),
                claims: Default::default(),
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
                test_case.claims,
                Json::from(UpdatePrayerTimesRequest {
                    prayer_times_data: test_case.prayer_times_data.clone(),
                    hash: "a13132143143134242".to_owned(),
                }),
            );
        }
    }
}
