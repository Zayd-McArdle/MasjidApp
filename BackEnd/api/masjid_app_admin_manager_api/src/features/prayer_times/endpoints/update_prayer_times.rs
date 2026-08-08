use crate::features::prayer_times::errors::update_prayer_times_repository_error::UpdatePrayerTimesRepositoryError;
use crate::features::prayer_times::models::update_prayer_times_request::UpdatePrayerTimesRequest;
use crate::features::prayer_times::services::errors::update_prayer_times_service_error::UpdatePrayerTimesServiceError;
use crate::features::prayer_times::services::prayer_times_update_service::PrayerTimesUpdateService;
use crate::shared::jwt::Claims;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use validator::Validate;

pub async fn update_prayer_times(
    State(state): State<ServiceAppState<Arc<dyn PrayerTimesUpdateService>>>,
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
    match state.service.update_prayer_times(prayer_times).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(UpdatePrayerTimesServiceError::RepositoryError(
            UpdatePrayerTimesRepositoryError::UnableToUpdatePrayerTimes,
        )) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::prayer_times::services::prayer_times_update_service::MockPrayerTimesUpdateService;

    #[tokio::test]
    async fn test_update_prayer_times() {
        struct TestCase {
            cached_prayer_times_data: Vec<u8>,
            prayer_times_data: Vec<u8>,
            expected_service_response: Option<Result<(), UpdatePrayerTimesServiceError>>,
            expected_api_response_code: StatusCode,
            claims: Claims,
        }
        let test_cases = vec![
            TestCase {
                cached_prayer_times_data: vec![],
                prayer_times_data: vec![],
                expected_api_response_code: StatusCode::BAD_REQUEST,
                expected_service_response: None,
                claims: Default::default(),
            },
            TestCase {
                cached_prayer_times_data: vec![1, 2, 3, 4, 5],
                prayer_times_data: vec![1, 2, 3, 4, 5],
                expected_api_response_code: StatusCode::INTERNAL_SERVER_ERROR,
                expected_service_response: Some(Err(
                    UpdatePrayerTimesServiceError::RepositoryError(
                        UpdatePrayerTimesRepositoryError::UnableToUpdatePrayerTimes,
                    ),
                )),
                claims: Default::default(),
            },
            TestCase {
                cached_prayer_times_data: vec![1, 2, 3, 4, 5],
                prayer_times_data: vec![1, 2, 3, 4, 5],
                expected_api_response_code: StatusCode::OK,
                expected_service_response: Some(Ok(())),
                claims: Default::default(),
            },
        ];
        for test_case in test_cases {
            let mut mock_service = MockPrayerTimesUpdateService::new();
            if let Some(expected_service_response) = test_case.expected_service_response {
                mock_service
                    .expect_update_prayer_times()
                    .return_once(move |data| expected_service_response);
            }
            let app_state = ServiceAppState::<Arc<dyn PrayerTimesUpdateService>> {
                service: Arc::new(mock_service),
            };
            let actual_response = update_prayer_times(
                State(app_state),
                test_case.claims,
                Json::from(UpdatePrayerTimesRequest {
                    prayer_times_data: test_case.prayer_times_data.clone(),
                    hash: "a13132143143134242".to_owned(),
                }),
            );
            assert!(matches!(
                test_case.expected_api_response_code,
                actual_response
            ));
        }
    }
}
