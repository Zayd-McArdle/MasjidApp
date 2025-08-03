use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use masjid_app_api_library::features::events::{Event, EventDetails, EventsRepository};
use masjid_app_api_library::shared::app_state::DbType;
use masjid_app_api_library::shared::repository_manager::{MySqlRepository, RepositoryType};

pub struct PostEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub event_details: EventDetails
}
#[async_trait]
pub trait EventsAdminRepository: EventsRepository {
    async fn post_event(&self, event: Event);
    async fn edit_event(&self, event: Event);
    async fn delete_event(&self, event_id: i32);
}
#[async_trait]
impl EventsAdminRepository for MySqlRepository {
    async fn post_event(&self, event: Event) {
        todo!()
    }

    async fn edit_event(&self, event: Event) {
        todo!()
    }

    async fn delete_event(&self, event_id: i32) {
        todo!()
    }
}
pub async fn new_events_admin_repository(db_type: DbType) -> Arc<dyn EventsAdminRepository> {
    Arc::new(MySqlRepository::new(RepositoryType::Events).await)
}