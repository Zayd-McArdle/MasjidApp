use axum::extract::State;
use axum::response::Response;
use masjid_app_api_library::features::events::{get_events_common, EventsRepository};
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::data_access::repository_manager::{
    InMemoryRepository, MySqlRepository, RepositoryType,
};
use masjid_app_api_library::shared::types::app_state::AppState;
use std::sync::Arc;

pub async fn new_events_public_repository(db_type: DbType) -> Arc<dyn EventsRepository> {
    match db_type {
        DbType::InMemory => Arc::new(InMemoryRepository::new(RepositoryType::Events).await),
        DbType::MySql => Arc::new(MySqlRepository::new(RepositoryType::Events).await),
    }
}
pub async fn get_events(State(state): State<AppState<Arc<dyn EventsRepository>>>) -> Response {
    get_events_common(State(state)).await
}
