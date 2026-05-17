use masjid_app_api_library::features::events::repositories::EventsRepository;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use masjid_app_api_library::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use masjid_app_api_library::shared::data_access::repository_manager::{
    InMemoryRepository, MySqlRepository, RepositoryType,
};
use masjid_app_api_library::shared::data_access::repository_mode::RepositoryMode;
use std::sync::Arc;

pub async fn new_events_public_repository(
    repository_mode: RepositoryMode,
) -> Arc<dyn EventsRepository> {
    new_repository!(repository_mode, RepositoryType::Events)
}
