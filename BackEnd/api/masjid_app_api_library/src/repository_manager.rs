use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::{Error, MySql, Pool};
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
    pub db_connection: Arc<MySqlPool>,
}
impl MySqlRepository {
    pub async fn new(repository_type: RepositoryType) -> Self {
        let connection_string_environment_variable = match repository_type {
            RepositoryType::Authentication => {
                tracing::info!("establishing database connection for authenticating users");
                "AUTHENTICATION_CONNECTION"
            }
            RepositoryType::PrayerTimes => {
                tracing::info!("establishing database connection for retrieving prayer times");
                "PRAYER_TIMES_CONNECTION"
            }
            RepositoryType::Announcement => {
                tracing::info!("establishing database connection for retrieving announcements");
                "ANNOUNCEMENT_CONNECTION"
            }
        };
        let connection_string = std::env::var(connection_string_environment_variable).unwrap();
        let db_connection_result = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&connection_string)
            .await;
        match db_connection_result {
            Ok(db_connection) => {
                tracing::info!("database connection successfully established");
                Self {
                    db_connection: Arc::new(db_connection),
                }
            }
            Err(err) => {
                panic!("Failed to connect to database: {err}");
            }
        }
    }
}
