use crate::features::events::repositories::EventsAdminRepository;
use crate::features::events::repositories::errors::delete_event_error::DeleteEventError;
use crate::features::events::repositories::errors::upsert_event_error::UpsertEventError;
use async_trait::async_trait;
use masjid_app_api_library::features::events::models::event::Event;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;

#[async_trait]
impl EventsAdminRepository for InMemoryRepository {
    async fn upsert_event(&self, event: &Event) -> Result<(), UpsertEventError> {
        tracing::warn!("in-memory database for upserting event not implemented");
        Err(UpsertEventError::UnableToUpsertEvent)
    }

    async fn delete_event_by_id(&self, event_id: &i32) -> Result<Option<String>, DeleteEventError> {
        tracing::warn!("in-memory database for deleting event not implemented");
        Err(DeleteEventError::UnableToDeleteEvent)
    }
}
