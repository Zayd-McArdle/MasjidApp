use std::sync::Arc;
use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use mockall::automock;
use mockall::predicate::ge;
use serde::{Deserialize, Serialize};
use crate::features::prayer_times::{build_prayer_times_response, GetPrayerTimesError, PrayerTimesDTO};
use crate::shared::age_range::AgeRange;
use crate::shared::app_state::{AppState, DbType};
use crate::shared::contact_details::ContactDetails;
use crate::shared::repository_manager::{InMemoryRepository, MySqlRepository};


#[derive(Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum EventStatus {
    Planned,
    Confirmed,
    Cancelled,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventType {
    Talk
}

#[derive(Clone, Debug, PartialEq)]
pub struct EventDetails {
    pub event_type: EventType,
    pub age_range: AgeRange,
    pub image: Option<Vec<u8>>,
    pub contact_details: ContactDetails,
}

#[derive(sqlx::FromRow, Serialize, Clone, Debug, PartialEq)]
pub struct EventDTO {
    pub id: i32,
    pub title: String,
    pub date: DateTime<Utc>,
    pub event_details: EventDetails
}
pub enum GetEventsError {
    EventsNotFound,
    UnableToGetEvents
}

#[automock]
#[async_trait]
pub trait EventsRepository : Send + Sync {
    async fn get_events() -> Result<Vec<EventDTO>, GetEventsError>;
}
impl EventsRepository for InMemoryRepository {
    async fn get_events() -> Result<Vec<EventDTO>, GetEventsError> {
        tracing::warn!("In-memory database for getting events not implemented");
        Err(GetEventsError::UnableToGetEvents)
    }
}
impl EventsRepository for MySqlRepository {
    async fn get_events() -> Result<Vec<EventDTO>, GetEventsError> {
        todo!()
    }
}
async fn get_events_common<R>(State(state): State<AppState<Arc<R>>>) -> Response
where R: EventsRepository{
    let mut get_events_result: Result<Vec<EventDTO>, GetEventsError> = Err(GetEventsError::UnableToGetEvents);

    if let Some(events_in_memory_repository) = state.repository_map.get(&DbType::InMemory) {
        get_events_result = events_in_memory_repository.get_prayer_times().await;
    }

    if get_events_result.is_err() {
        get_events_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .get_prayer_times()
            .await;
    }
    match get_events_result {
        Ok(events) => (StatusCode::OK, events).into_response(),
        Err(GetEventsError::EventsNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(GetEventsError::UnableToGetEvents) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

mod test {
    use super::*;
    use crate::shared::app_state::AppState;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_events() {
        struct TestCase {
            cached_prayer_times_data: Result<EventDTO, GetEventsError>,
            prayer_times_data: Result<EventDTO, GetEventsError>,
            expected_response_code: StatusCode,
        };
        let valid_event_data = EventDTO {
            id: 1,
            title: "An Event Title".to_string(),
            date: DateTime::default(),
            age_range: AgeRange { minimum_age: 16, maximum_age: 19 },
            image: None,
            contact_details: None,
        };
    }
}