use crate::features::events::models::event_dto::EventDTO;
use crate::features::events::repositories::EventsRepository;
use crate::features::events::repositories::errors::get_events_repository_error::GetEventsRepositoryError;
use crate::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use async_trait::async_trait;

#[async_trait]
impl EventsRepository for InMemoryRepository {
    async fn get_events(&self) -> Result<Vec<EventDTO>, GetEventsRepositoryError> {
        tracing::warn!("In-memory database for getting events not implemented");
        Err(GetEventsRepositoryError::UnableToGetEvents)
    }
}
