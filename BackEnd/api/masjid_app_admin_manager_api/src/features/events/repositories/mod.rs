use crate::features::events::repositories::errors::delete_event_error::DeleteEventError;
use crate::features::events::repositories::errors::upsert_event_error::UpsertEventError;
use async_trait::async_trait;
use masjid_app_api_library::features::events::models::event::Event;
use masjid_app_api_library::features::events::repositories::EventsRepository;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use masjid_app_api_library::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use masjid_app_api_library::shared::data_access::repository_management::repository_mode::RepositoryMode;
use masjid_app_api_library::shared::data_access::repository_management::repository_type::RepositoryType;
use std::sync::Arc;

pub mod errors;
mod mysql_impl;
mod redis_impl;
#[async_trait]
pub trait EventsAdminRepository: EventsRepository {
    async fn upsert_event(&self, event: &Event) -> Result<(), UpsertEventError>;
    async fn delete_event_by_id(&self, event_id: &i32) -> Result<Option<String>, DeleteEventError>;
}

pub async fn new_events_admin_repository(
    repository_mode: RepositoryMode,
) -> Arc<dyn EventsAdminRepository> {
    new_repository!(repository_mode, RepositoryType::Events)
}
