use std::sync::Arc;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
#[derive(Hash, Eq, PartialEq)]
pub enum ConnectionString {
    AuthenticationConnection,
    PrayerTimesConnection,
    AnnouncementConnection,
}

#[derive(PartialEq)]
pub enum RepositoryMode {
    InMemory,
    Normal,
}
pub struct MainRepository {
    pub(crate) db_connection : Arc<MySqlPool>
}
impl MainRepository {
    pub async fn new(connection_string: ConnectionString) -> Self {
        let connection_string_environment_variable  = match connection_string {
            ConnectionString::AuthenticationConnection => "AUTHENTICATION_CONNECTION",
            ConnectionString::PrayerTimesConnection => "PRAYER_TIMES_CONNECTION",
            ConnectionString::AnnouncementConnection => "ANNOUNCEMENT_CONNECTION",
        };
        let db_connection = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(std::env::var(connection_string_environment_variable).unwrap().as_str())
            .await.unwrap();
        MainRepository {db_connection: Arc::new(db_connection)}
    }
}