use masjid_app_api_library::features::events::repository::EventsRepository;
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::data_access::repository_manager::{
    InMemoryRepository, MySqlRepository, RepositoryType,
};
use std::sync::Arc;

pub async fn new_events_public_repository(db_type: DbType) -> Arc<dyn EventsRepository> {
    match db_type {
        DbType::InMemory => Arc::new(InMemoryRepository::new(RepositoryType::Events).await),
        DbType::MySql => Arc::new(MySqlRepository::new(RepositoryType::Events).await),
    }
}
