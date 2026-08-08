use crate::features::events::models::event::Event;
use crate::features::events::models::event_dto::EventDTO;
use crate::features::events::repositories::EventsRepository;
use crate::features::events::repositories::errors::get_events_repository_error::GetEventsRepositoryError;
use crate::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use async_trait::async_trait;
use sqlx::mysql::MySqlRow;
use sqlx::{Error, Row};

#[async_trait]
impl EventsRepository for MySqlRepository {
    async fn get_events(&self) -> Result<Vec<EventDTO>, GetEventsRepositoryError> {
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
                    return GetEventsRepositoryError::EventsNotFound;
                }
                tracing::error!("failed to fetch events from database: {}", err);
                GetEventsRepositoryError::UnableToGetEvents
            })?;
        if events.is_empty() {
            return Err(GetEventsRepositoryError::EventsNotFound);
        }

        Ok(events.into_iter().map(EventDTO::from).collect())
    }
}
