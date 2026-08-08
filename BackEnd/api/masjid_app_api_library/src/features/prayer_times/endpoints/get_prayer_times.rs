use crate::features::prayer_times::endpoints::utils::build_prayer_times_response;
use crate::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;
use crate::features::prayer_times::services::errors::get_prayer_times_service_error::GetPrayerTimesServiceError;
use crate::features::prayer_times::services::prayer_times_retrieval_service::PrayerTimesRetrievalService;
use crate::shared::types::app_state::ServiceAppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;

pub async fn get_prayer_times_common<R: PrayerTimesRetrievalService + ?Sized>(
    State(state): State<ServiceAppState<Arc<R>>>,
) -> Response {
    match state.service.get_prayer_times().await {
        Ok(prayer_times) => build_prayer_times_response(prayer_times, None),
        Err(GetPrayerTimesServiceError::RepositoryError(
            GetPrayerTimesRepositoryError::PrayerTimesNotFound,
        )) => StatusCode::NOT_FOUND.into_response(),
        Err(GetPrayerTimesServiceError::RepositoryError(
            GetPrayerTimesRepositoryError::UnableToGetPrayerTimes,
        )) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
    use crate::features::prayer_times::services::prayer_times_retrieval_service::MockPrayerTimesRetrievalService;

    #[tokio::test]
    async fn test_get_prayer_times() {
        struct TestCase {
            expected_service_response: Result<PrayerTimesDTO, GetPrayerTimesServiceError>,
            expected_response_code: StatusCode,
        }
        let valid_prayer_times_data = Ok(PrayerTimesDTO {
            data: Some(vec![1, 2, 3, 4, 5]),
            hash: "5a4e9c5d6b8a2f3e1c0b9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d7".to_owned(),
        });
        let test_cases = vec![
            TestCase {
                expected_service_response: Err(GetPrayerTimesServiceError::RepositoryError(
                    GetPrayerTimesRepositoryError::PrayerTimesNotFound,
                )),
                expected_response_code: StatusCode::NOT_FOUND,
            },
            TestCase {
                expected_service_response: Err(GetPrayerTimesServiceError::RepositoryError(
                    GetPrayerTimesRepositoryError::UnableToGetPrayerTimes,
                )),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                expected_service_response: valid_prayer_times_data,
                expected_response_code: StatusCode::OK,
            },
        ];

        for case in test_cases {
            let mut mock_service = MockPrayerTimesRetrievalService::new();

            mock_service
                .expect_get_prayer_times()
                .return_once(move || case.expected_service_response);

            let app_state = ServiceAppState::<Arc<dyn PrayerTimesRetrievalService>> {
                service: Arc::new(mock_service),
            };

            let actual_response = get_prayer_times_common(State(app_state)).await;

            // Assert response matches expected status code
            assert_eq!(case.expected_response_code, actual_response.status());
        }
    }
}
