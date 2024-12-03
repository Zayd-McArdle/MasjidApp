use std::sync::Arc;
use sqlx::mysql;
use std::collections::HashMap;
use std::ffi::CString;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
#[derive(Hash, Eq, PartialEq)]
pub enum ConnectionString {
    AuthenticationConnection,
    PrayerTimesConnection,
    AnnouncementConnection,
}
const connection_string_mapper: HashMap<ConnectionString, &str> = HashMap::from([
    (ConnectionString::AuthenticationConnection, "AUTHENTICATION_CONNECTION"),
    (ConnectionString::PrayerTimesConnection, "PRAYER_TIMES_CONNECTION"),
    (ConnectionString::AnnouncementConnection, "ANNOUNCEMENT_CONNECTION"),
]);

pub struct Repository {
    pub(crate) db_connection : MySqlPool
}
impl Repository {
    pub async fn new(connection_string: ConnectionString) -> Self {
        let db_connection = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(std::env::var(connection_string_mapper.get(&connection_string).unwrap()).unwrap().as_str())
            .await.unwrap();
        Repository{db_connection}
    }
}