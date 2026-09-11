use crate::features::prayer_times::models::update_prayer_times_request::UpdatePrayerTimesRequest;
use crate::features::prayer_times::services::prayer_times_update_service::PrayerTimesUpdateService;
use crate::shared::jwt::Claims;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use masjid_app_api_library::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use sha2::{Digest, Sha256};
use std::borrow::ToOwned;
use std::sync::Arc;
use validator::Validate;
const PRAYER_TIMES_VERIFICATION_FAILED: &'static str = "Verification of prayer times failed";

const UNABLE_TO_UPDATE_PRAYER_TIMES: &'static str = "Unable to update prayer times";
pub async fn update_prayer_times(
    State(state): State<ServiceAppState<Arc<dyn PrayerTimesUpdateService>>>,
    _claims: Claims,
    Json(request): Json<UpdatePrayerTimesRequest>,
) -> Result<(), (StatusCode, String)> {
    request.validate().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid JSON request: {request:?}").to_owned(),
        )
    })?;
    let hashed_prayer_times = format!("{:x}", Sha256::digest(&request.prayer_times_data));
    if request.hash != hashed_prayer_times {
        return Err((
            StatusCode::BAD_REQUEST,
            PRAYER_TIMES_VERIFICATION_FAILED.to_owned(),
        ));
    }
    let prayer_times = PrayerTimesDTO {
        data: Some(request.prayer_times_data),
        hash: request.hash,
    };
    state
        .service
        .update_prayer_times(prayer_times)
        .await
        .map_err(move |err| {
            tracing::error!(err = ?err, UNABLE_TO_UPDATE_PRAYER_TIMES);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                UNABLE_TO_UPDATE_PRAYER_TIMES.to_owned(),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::prayer_times::errors::update_prayer_times_repository_error::UpdatePrayerTimesRepositoryError;
    use crate::features::prayer_times::services::errors::update_prayer_times_service_error::UpdatePrayerTimesServiceError;
    use crate::features::prayer_times::services::prayer_times_update_service::MockPrayerTimesUpdateService;

    #[tokio::test]
    async fn test_update_prayer_times() {
        const HASH: &'static str =
            "74f81fe167d99b4cb41d6d0ccda82278caee9f3e2f25d5e5a3936ff3dcec60d0H";
        struct TestCase {
            prayer_times_data: Vec<u8>,
            expected_service_response: Option<Result<(), UpdatePrayerTimesServiceError>>,
            expected_result: Result<(), (StatusCode, String)>,
        }
        let bad_prayer_times_request = UpdatePrayerTimesRequest {
            prayer_times_data: vec![],
            hash: HASH.to_owned(),
        };
        let test_cases = vec![
            TestCase {
                prayer_times_data: vec![],
                expected_result: Err((
                    StatusCode::BAD_REQUEST,
                    format!("Invalid JSON request: {bad_prayer_times_request:?}"),
                )),
                expected_service_response: None,
            },
            TestCase {
                prayer_times_data: vec![1, 2, 3, 4, 5],
                expected_result: Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    UNABLE_TO_UPDATE_PRAYER_TIMES.to_owned(),
                )),
                expected_service_response: Some(Err(
                    UpdatePrayerTimesServiceError::RepositoryError(
                        UpdatePrayerTimesRepositoryError::UnableToUpdatePrayerTimes,
                    ),
                )),
            },
            TestCase {
                prayer_times_data: vec![1, 2, 3, 4, 5],
                expected_result: Ok(()),
                expected_service_response: Some(Ok(())),
            },
        ];
        for test_case in test_cases {
            let mut mock_service = MockPrayerTimesUpdateService::new();
            if let Some(expected_service_response) = test_case.expected_service_response {
                mock_service
                    .expect_update_prayer_times()
                    .return_once(move |_| expected_service_response);
            }
            let app_state = ServiceAppState::<Arc<dyn PrayerTimesUpdateService>> {
                service: Arc::new(mock_service),
            };
            let actual_result = update_prayer_times(
                State(app_state),
                Claims::default(),
                Json(UpdatePrayerTimesRequest {
                    prayer_times_data: test_case.prayer_times_data.clone(),
                    hash: "74f81fe167d99b4cb41d6d0ccda82278caee9f3e2f25d5e5a3936ff3dcec60d0"
                        .to_owned(),
                }),
            )
            .await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
