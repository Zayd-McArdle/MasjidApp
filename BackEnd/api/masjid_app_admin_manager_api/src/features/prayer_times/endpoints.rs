use crate::features::prayer_times::errors::UpdatePrayerTimesError;
use crate::features::prayer_times::models::UpdatePrayerTimesRequest;
use crate::features::prayer_times::repositories::PrayerTimesAdminRepository;
use crate::shared::jwt::Claims;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use masjid_app_api_library::features::prayer_times::endpoints::get_prayer_times_common;
use masjid_app_api_library::features::prayer_times::models::PrayerTimesDTO;
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::types::app_state::AppState;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use validator::Validate;

pub async fn get_prayer_times(
    State(state): State<AppState<Arc<dyn PrayerTimesAdminRepository>>>,
    claims: Claims,
) -> Response {
    get_prayer_times_common(State(state)).await
}
pub async fn update_prayer_times(
    State(state): State<AppState<Arc<dyn PrayerTimesAdminRepository>>>,
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
        .get(&DbType::MySql)
        .unwrap()
        .update_prayer_times(prayer_times.clone())
        .await;
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
    use async_trait::async_trait;
    use masjid_app_api_library::features::prayer_times::errors::GetPrayerTimesError;
    use masjid_app_api_library::features::prayer_times::repositories::PrayerTimesRepository;
    use mockall::mock;
    use std::collections::HashMap;

    mock! {
        pub PrayerTimesAdminRepository {}

        // Implement the base trait
        #[async_trait]
        impl PrayerTimesRepository for PrayerTimesAdminRepository {
            async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError>;
        }

        // Implement the admin trait
        #[async_trait]
        impl PrayerTimesAdminRepository for PrayerTimesAdminRepository {
            async fn update_prayer_times(&self, prayer_times_data: PrayerTimesDTO) -> Result<(), UpdatePrayerTimesError>;
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
            let mut mock_prayer_times_repository = MockPrayerTimesAdminRepository::new();
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_prayer_times_repository
                    .expect_update_prayer_times()
                    .returning(move |data| expected_db_response.clone());
            }
            let arc_repository: Arc<dyn PrayerTimesAdminRepository> =
                Arc::new(mock_prayer_times_repository);
            let app_state: AppState<Arc<dyn PrayerTimesAdminRepository>> = AppState {
                repository_map: HashMap::from([(DbType::MySql, arc_repository)]),
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
