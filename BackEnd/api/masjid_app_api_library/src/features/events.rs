use crate::shared::age_range::AgeRange;
use crate::shared::app_state::{AppState, DbType};
use crate::shared::contact_details::ContactDetails;
use crate::shared::repository_manager::{InMemoryRepository, MySqlRepository};
use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Utc};
use mockall::automock;
use mockall::predicate::ge;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{Error, Row};
use std::sync::Arc;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, PartialEq)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Confirmed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Talk,
    Social,
    Class,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventRecurrence {
    Daily,
    Weekly,
    Fortnightly,
    Monthly,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Validate)]
pub struct EventDetails {
    pub event_type: EventType,
    pub event_recurrence: Option<EventRecurrence>,
    pub event_status: EventStatus,
    #[validate(nested)]
    pub age_range: Option<AgeRange>,
    #[validate(nested)]
    pub contact_details: ContactDetails,
}
#[derive(sqlx::FromRow, Clone, Debug, PartialEq)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    // Event Details
    pub r#type: EventType,
    pub recurrence: Option<EventRecurrence>,
    pub status: EventStatus,
    pub minimum_age: Option<u8>,
    pub maximum_age: Option<u8>,
    // Organiser Contact Details
    pub full_name: String,
    pub phone_number: String,
    pub email: Option<String>,
}
impl From<EventDTO> for Event {
    fn from(dto: EventDTO) -> Self {
        let (minimum_age, maximum_age): (Option<u8>, Option<u8>) = match dto.event_details.age_range
        {
            None => (None, None),
            Some(age_range) => (Some(age_range.minimum_age), Some(age_range.maximum_age)),
        };

        Self {
            id: dto.id,
            title: dto.title,
            description: dto.description,
            date: dto.date,
            r#type: dto.event_details.event_type,
            recurrence: dto.event_details.event_recurrence,
            status: dto.event_details.event_status,
            minimum_age: minimum_age,
            maximum_age: maximum_age,
            full_name: dto.event_details.contact_details.full_name,
            phone_number: dto.event_details.contact_details.phone_number,
            email: dto.event_details.contact_details.email,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EventDTO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub event_details: EventDetails,
}

impl From<Event> for EventDTO {
    fn from(event: Event) -> Self {
        let mut age_range: Option<AgeRange> = None;
        if event.minimum_age.is_some() && event.maximum_age.is_some() {
            age_range = Some(AgeRange {
                minimum_age: event.minimum_age.unwrap(),
                maximum_age: event.maximum_age.unwrap(),
            });
        }
        Self {
            id: event.id,
            title: event.title,
            description: event.description,
            date: event.date,
            event_details: EventDetails {
                event_type: event.r#type,
                event_recurrence: event.recurrence,
                event_status: event.status,
                age_range: age_range,
                contact_details: ContactDetails {
                    full_name: event.full_name,
                    phone_number: event.phone_number,
                    email: event.email,
                },
            },
        }
    }
}

#[derive(Clone)]
pub enum GetEventsError {
    EventsNotFound,
    UnableToGetEvents,
}

#[automock]
#[async_trait]
pub trait EventsRepository: Send + Sync {
    async fn get_events(&self) -> Result<Vec<EventDTO>, GetEventsError>;
}
#[async_trait]
impl EventsRepository for InMemoryRepository {
    async fn get_events(&self) -> Result<Vec<EventDTO>, GetEventsError> {
        tracing::warn!("In-memory database for getting events not implemented");
        Err(GetEventsError::UnableToGetEvents)
    }
}
#[async_trait]
impl EventsRepository for MySqlRepository {
    async fn get_events(&self) -> Result<Vec<EventDTO>, GetEventsError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL get_events();")
            .map(|row: MySqlRow| Event {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                date: row.get(3),
                r#type: row.get(4),
                recurrence: row.get(5),
                status: row.get(6),
                minimum_age: row.get(7),
                maximum_age: row.get(8),
                full_name: row.get(9),
                phone_number: row.get(10),
                email: row.get(11),
            })
            .fetch_all(&*db_connection)
            .await;
        match query_response {
            Ok(events) => Ok(events.into_iter().map(EventDTO::from).collect()),
            Err(Error::RowNotFound) => Err(GetEventsError::EventsNotFound),
            Err(err) => {
                tracing::error!("Error getting events: {err}");
                Err(GetEventsError::UnableToGetEvents)
            }
        }
    }
}
async fn get_events_common<R>(State(state): State<AppState<Arc<R>>>) -> Response
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
    use crate::shared::app_state::AppState;
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
                event_recurrence: None,
                event_status: EventStatus::Confirmed,
                age_range: Some(AgeRange {
                    minimum_age: 16,
                    maximum_age: 18,
                }),
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
