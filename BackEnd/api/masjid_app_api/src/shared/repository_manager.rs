use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::sync::Arc;
#[derive(Hash, Eq, PartialEq)]
pub enum RepositoryType {
    Authentication,
    PrayerTimes,
    Announcement,
}

#[derive(PartialEq)]
pub enum RepositoryMode {
    InMemory,
    Normal,
}
pub struct InMemoryRepository {}

impl InMemoryRepository {
    pub async fn new(repository_type: RepositoryType) -> Self {
        InMemoryRepository {}
    }
}
pub struct MySqlRepository {
    pub(crate) db_connection: Arc<MySqlPool>,
}
impl MySqlRepository {
    pub async fn new(repository_type: RepositoryType) -> Self {
        let connection_string_environment_variable = match repository_type {
            RepositoryType::Authentication => "AUTHENTICATION_CONNECTION",
            RepositoryType::PrayerTimes => "PRAYER_TIMES_CONNECTION",
            RepositoryType::Announcement => "ANNOUNCEMENT_CONNECTION",
        };
        let connection_string = std::env::var(connection_string_environment_variable).unwrap();
        let db_connection = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&connection_string)
            .await
            .unwrap();
        MySqlRepository {
            db_connection: Arc::new(db_connection),
        }
    }
}
