use crate::features::prayer_times::services::errors::check_for_updated_prayer_times_error::CheckForUpdatedPrayerTimesError;
use crate::features::prayer_times::services::prayer_times_update_checking_service::PrayerTimesUpdateCheckingService;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::features::prayer_times::endpoints::{
    build_prayer_times_response, get_prayer_times_common,
};
use masjid_app_api_library::features::prayer_times::errors::GetPrayerTimesRepositoryError;
use masjid_app_api_library::features::prayer_times::services::prayer_times_retrieval_service::PrayerTimesRetrievalService;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

pub async fn get_prayer_times(
    State(state): State<ServiceAppState<Arc<dyn PrayerTimesRetrievalService>>>,
) -> Response {
    get_prayer_times_common(State(state)).await
}

pub async fn get_updated_prayer_times(
    State(state): State<ServiceAppState<Arc<dyn PrayerTimesUpdateCheckingService>>>,
    hash: Path<String>,
) -> Response {
    if hash.len() != 64 {
        return (
            StatusCode::BAD_REQUEST,
            format!("Malformed hash: {}", hash.0),
        )
            .into_response();
    }

    match state.service.check_for_updated_prayer_times(&hash).await {
        Ok(prayer_times) => build_prayer_times_response(prayer_times, Some(&hash)),
        Err(CheckForUpdatedPrayerTimesError::RepositoryError(
            GetPrayerTimesRepositoryError::PrayerTimesNotFound,
        )) => StatusCode::NOT_FOUND.into_response(),
        Err(CheckForUpdatedPrayerTimesError::RepositoryError(
            GetPrayerTimesRepositoryError::UnableToGetPrayerTimes,
        )) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::prayer_times::repositories::PrayerTimesPublicRepository;
    use async_trait::async_trait;
    use masjid_app_api_library::features::prayer_times::models::PrayerTimesDTO;
    use masjid_app_api_library::features::prayer_times::repositories::PrayerTimesRepository;
    use masjid_app_api_library::features::prayer_times::services::errors::get_prayer_times_service_error::GetPrayerTimesServiceError;
    use masjid_app_api_library::features::prayer_times::services::prayer_times_retrieval_service::MockPrayerTimesRetrievalService;
    use mockall::mock;

    mock!(
        pub PrayerTimesPublicRepository {}

        // Implement the base trait
        #[async_trait]
        impl PrayerTimesRepository for PrayerTimesPublicRepository {
            async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesRepositoryError>;
        }

        #[async_trait]
        impl PrayerTimesPublicRepository for PrayerTimesPublicRepository {
            async fn get_updated_prayer_times(&self, hash: &str) -> Result<PrayerTimesDTO, GetPrayerTimesRepositoryError>;
        }
    );

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
        let test_cases = [
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

        for test_case in test_cases {
            let mut mock_service = MockPrayerTimesRetrievalService::new();

            mock_service
                .expect_get_prayer_times()
                .return_once(move || test_case.expected_service_response);

            let app_state = ServiceAppState::<Arc<dyn PrayerTimesRetrievalService>> {
                service: Arc::new(mock_service),
            };

            let actual_response = get_prayer_times(State(app_state)).await;

            // Assert response matches expected status code
            assert_eq!(test_case.expected_response_code, actual_response.status());
        }
    }
}
