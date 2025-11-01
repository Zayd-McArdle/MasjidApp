use async_trait::async_trait;
use mockall::automock;
use sqlx::mysql::MySqlRow;
use sqlx::{Error, Row};
use crate::features::events::errors::GetEventsError;
use crate::features::events::models::{Event, EventDTO};
use crate::shared::data_access::repository_manager::{InMemoryRepository, MySqlRepository};

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