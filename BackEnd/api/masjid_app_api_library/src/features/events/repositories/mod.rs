use crate::features::events::models::event_dto::EventDTO;
use crate::features::events::repositories::errors::get_events_repository_error::GetEventsRepositoryError;
use async_trait::async_trait;
use mockall::automock;

pub mod errors;
mod mysql_impl;
mod redis_impl;

#[automock]
#[async_trait]
pub trait EventsRepository: Send + Sync {
    async fn get_events(&self) -> Result<Vec<EventDTO>, GetEventsRepositoryError>;
}
