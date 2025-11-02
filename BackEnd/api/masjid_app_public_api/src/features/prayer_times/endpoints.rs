pub async fn get_prayer_times(
    State(state): State<AppState<Arc<dyn PrayerTimesPublicRepository>>>,
) -> Response {
    get_prayer_times_common(State(state)).await
}

pub async fn get_updated_prayer_times(
    State(state): State<AppState<Arc<dyn PrayerTimesPublicRepository>>>,
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
#[cfg(test)]
mod tests {
    use super::*;
    use masjid_app_api_library::shared::types::app_state::AppState;
    use std::collections::HashMap;

    mock!(
        pub PrayerTimesPublicRepository {}

        // Implement the base trait
        #[async_trait]
        impl PrayerTimesRepository for PrayerTimesPublicRepository {
            async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError>;
        }

        #[async_trait]
        impl PrayerTimesPublicRepository for PrayerTimesPublicRepository {
            async fn get_updated_prayer_times(&self, hash: &str) -> Result<PrayerTimesDTO, GetPrayerTimesError>;
        }
    );

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
            let mut mock_prayer_times_in_memory_repository = MockPrayerTimesPublicRepository::new();
            let mut mock_prayer_times_repository = MockPrayerTimesPublicRepository::new();

            mock_prayer_times_in_memory_repository
                .expect_get_prayer_times()
                .returning(move || case.cached_prayer_times_data.clone());
            mock_prayer_times_repository
                .expect_get_prayer_times()
                .returning(move || case.prayer_times_data.clone());
            let arc_in_memory_repository: Arc<dyn PrayerTimesPublicRepository> =
                Arc::new(mock_prayer_times_in_memory_repository);
            let arc_repository: Arc<dyn PrayerTimesPublicRepository> =
                Arc::new(mock_prayer_times_repository);
            let app_state: AppState<Arc<dyn PrayerTimesPublicRepository>> = AppState {
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
}
