use crate::shared::data_access::db_type::DbType;
use crate::shared::data_access::repository_manager::{InMemoryRepository, MySqlRepository};
use crate::shared::types::age_range::AgeRange;
use crate::shared::types::app_state::AppState;
use crate::shared::types::contact_details::ContactDetails;
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
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, PartialEq, Eq)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Confirmed,
    Cancelled,
}
impl ToString for EventStatus {
    fn to_string(&self) -> String {
        match self {
            EventStatus::Confirmed => "confirmed".to_owned(),
            EventStatus::Cancelled => "cancelled".to_owned(),
        }
    }
}
impl FromStr for EventStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "confirmed" => Ok(EventStatus::Confirmed),
            "cancelled" => Ok(EventStatus::Cancelled),
            _ => Err(()),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::Type, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Talk,
    Social,
    Class,
}
impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            EventType::Talk => "talk".to_owned(),
            EventType::Social => "social".to_owned(),
            EventType::Class => "class".to_owned(),
        }
    }
}

impl FromStr for EventType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "talk" => Ok(EventType::Talk),
            "social" => Ok(EventType::Social),
            "class" => Ok(EventType::Class),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
pub enum EventRecurrence {
    OneOff,
    Daily,
    Weekly,
    Fortnightly,
    Monthly,
}

impl ToString for EventRecurrence {
    fn to_string(&self) -> String {
        match self {
            EventRecurrence::OneOff => "one-off".to_owned(),
            EventRecurrence::Daily => "daily".to_owned(),
            EventRecurrence::Weekly => "weekly".to_owned(),
            EventRecurrence::Fortnightly => "fortnightly".to_owned(),
            EventRecurrence::Monthly => "monthly".to_owned(),
        }
    }
}

impl FromStr for EventRecurrence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one-off" => Ok(EventRecurrence::OneOff),
            "daily" => Ok(EventRecurrence::Daily),
            "weekly" => Ok(EventRecurrence::Weekly),
            "fortnight" => Ok(EventRecurrence::Fortnightly),
            "monthly" => Ok(EventRecurrence::Monthly),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Validate)]
pub struct EventDetails {
    #[serde(rename(serialize = "eventType", deserialize = "eventType"))]
    pub event_type: EventType,

    #[serde(rename(serialize = "eventRecurrence", deserialize = "eventRecurrence"))]
    pub event_recurrence: EventRecurrence,

    #[serde(rename(serialize = "eventStatus", deserialize = "eventStatus"))]
    pub event_status: EventStatus,

    #[validate(nested)]
    #[serde(rename(serialize = "ageRange", deserialize = "ageRange"))]
    pub age_range: Option<AgeRange>,

    #[validate(url)]
    #[serde(rename(serialize = "imageUrl", deserialize = "imageUrl"))]
    pub image_url: Option<String>,

    #[validate(nested)]
    #[serde(rename(serialize = "contactDetails", deserialize = "contactDetails"))]
    pub contact_details: ContactDetails,
}
#[derive(sqlx::FromRow, Clone, Debug, PartialEq)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    // Event Details
    pub r#type: String,
    pub recurrence: String,
    pub status: String,
    pub minimum_age: Option<u8>,
    pub maximum_age: Option<u8>,
    pub image_url: Option<String>,
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
            r#type: dto.event_details.event_type.to_string(),
            recurrence: dto.event_details.event_recurrence.to_string(),
            status: dto.event_details.event_status.to_string(),
            minimum_age: minimum_age,
            maximum_age: maximum_age,
            image_url: dto.event_details.image_url,
            full_name: dto.event_details.contact_details.full_name,
            phone_number: dto.event_details.contact_details.phone_number,
            email: dto.event_details.contact_details.email,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Validate)]
pub struct EventDTO {
    pub id: i32,

    #[validate(length(min = 4))]
    pub title: String,

    #[validate(length(min = 4))]
    pub description: Option<String>,

    pub date: DateTime<Utc>,

    #[validate(nested)]
    #[serde(rename(serialize = "eventDetails", deserialize = "eventDetails"))]
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
                event_type: EventType::from_str(&event.r#type).unwrap(),
                event_recurrence: EventRecurrence::from_str(&event.recurrence).unwrap(),
                event_status: EventStatus::from_str(&event.status).unwrap(),
                age_range: age_range,
                image_url: event.image_url,
                contact_details: ContactDetails {
                    full_name: event.full_name,
                    phone_number: event.phone_number,
                    email: event.email,
                },
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
        let events = sqlx::query("CALL get_events();")
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
                image_url: row.get(9),
                full_name: row.get(10),
                phone_number: row.get(11),
                email: row.get(12),
            })
            .fetch_all(&*db_connection)
            .await
            .map_err(|err| {
                if let Error::RowNotFound = err {
                    return GetEventsError::EventsNotFound;
                }
                tracing::error!("failed to fetch events from database: {}", err);
                GetEventsError::UnableToGetEvents
            })?;
        if events.is_empty() {
            return Err(GetEventsError::EventsNotFound);
        }

        Ok(events.into_iter().map(EventDTO::from).collect())
    }
}
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
