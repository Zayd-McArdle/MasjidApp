use crate::features::events::errors::GetEventsError;
use crate::features::events::models::EventDTO;
use crate::features::events::repository::EventsRepository;
use crate::shared::data_access::db_type::DbType;
use crate::shared::types::age_range::AgeRange;
use crate::shared::types::app_state::AppState;
use crate::shared::types::contact_details::ContactDetails;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;
use validator::Validate;

pub async fn get_events_common<R>(State(state): State<AppState<Arc<R>>>) -> Response
where
    R: EventsRepository + ?Sized,
{
    let mut get_events_result: Result<Vec<EventDTO>, GetEventsError> =
        Err(GetEventsError::UnableToGetEvents);

    if let Some(events_in_memory_repository) = state.repository_map.get(&DbType::InMemory) {
        get_events_result = events_in_memory_repository.get_events().await;
    }

    if get_events_result.is_err() {
        get_events_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .get_events()
            .await;
    }
    match get_events_result {
        Ok(events) => (StatusCode::OK, Json(events)).into_response(),
        Err(GetEventsError::EventsNotFound) => StatusCode::NO_CONTENT.into_response(),
        Err(GetEventsError::UnableToGetEvents) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
mod test {
    use super::*;
    use crate::features::events::models::{EventDetails, EventRecurrence, EventStatus, EventType};
    use crate::features::events::repository::{EventsRepository, MockEventsRepository};
    use crate::shared::types::app_state::AppState;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_events_common() {
        let events = vec![EventDTO {
            id: 0,
            title: "This is a title".to_owned(),
            description: Some("This is a description".to_owned()),
            date: Default::default(),
            event_details: EventDetails {
                event_type: EventType::Talk,
                event_recurrence: EventRecurrence::OneOff,
                event_status: EventStatus::Confirmed,
                age_range: Some(AgeRange {
                    minimum_age: 16,
                    maximum_age: 18,
                }),
                image_url: None,
                contact_details: ContactDetails {
                    full_name: "John Smith".to_owned(),
                    phone_number: "07127665431".to_owned(),
                    email: Some("johns.smith@masjidapp.com".to_owned()),
                },
            },
        }];
        struct TestCase {
            expected_in_memory_db_response: Result<Vec<EventDTO>, GetEventsError>,
            expected_db_response: Option<Result<Vec<EventDTO>, GetEventsError>>,
            expected_response_code: StatusCode,
        }
        let test_cases = vec![
            // When retrieval events fail on in-memory and MySQL database
            TestCase {
                expected_in_memory_db_response: Err(GetEventsError::UnableToGetEvents),
                expected_db_response: Some(Err(GetEventsError::UnableToGetEvents)),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            // When retrieval fails on in-memory database but finds no events in MySQL database
            TestCase {
                expected_in_memory_db_response: Err(GetEventsError::UnableToGetEvents),
                expected_db_response: Some(Err(GetEventsError::EventsNotFound)),
                expected_response_code: StatusCode::NO_CONTENT,
            },
            // When no events found for in-memory database but retrieval fails for MySQL database
            TestCase {
                expected_in_memory_db_response: Err(GetEventsError::EventsNotFound),
                expected_db_response: Some(Err(GetEventsError::UnableToGetEvents)),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            // When events found for in-memory database
            TestCase {
                expected_in_memory_db_response: Ok(events.clone()),
                expected_db_response: None,
                expected_response_code: StatusCode::OK,
            },
            // When events not found for in-memory database but found in MySQL database
            TestCase {
                expected_in_memory_db_response: Err(GetEventsError::EventsNotFound),
                expected_db_response: Some(Ok(events.clone())),
                expected_response_code: StatusCode::OK,
            },
            // When events found in MySQL database
            TestCase {
                expected_in_memory_db_response: Err(GetEventsError::UnableToGetEvents),
                expected_db_response: Some(Ok(events.clone())),
                expected_response_code: StatusCode::OK,
            },
        ];

        for case in test_cases {
            let mut mock_events_in_memory_repository = MockEventsRepository::new();
            let mut mock_events_repository = MockEventsRepository::new();

            mock_events_in_memory_repository
                .expect_get_events()
                .returning(move || case.expected_in_memory_db_response.clone());
            if let Some(expected_db_response) = case.expected_db_response {
                mock_events_repository
                    .expect_get_events()
                    .returning(move || expected_db_response.clone());
            }

            let arc_in_memory_repository: Arc<dyn EventsRepository> =
                Arc::new(mock_events_in_memory_repository);
            let arc_repository: Arc<dyn EventsRepository> = Arc::new(mock_events_repository);

            let app_state: AppState<Arc<dyn EventsRepository>> = AppState {
                repository_map: HashMap::from([
                    (DbType::InMemory, arc_in_memory_repository),
                    (DbType::MySql, arc_repository),
                ]),
            };

            let actual_response = get_events_common(State(app_state)).await;
            assert_eq!(actual_response.status(), case.expected_response_code)
        }
    }
}
