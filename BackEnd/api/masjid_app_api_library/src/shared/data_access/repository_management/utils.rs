use crate::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use crate::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use crate::shared::data_access::repository_management::repository_mode::RepositoryMode;
use crate::shared::data_access::repository_management::repository_type::RepositoryType;

pub const AUTHENTICATION_MYSQL_CONNECTION: &'static str = "AUTHENTICATION_CONNECTION";
pub const AUTHENTICATION_REDIS_CONNECTION: &'static str = "AUTHENTICATION_CONNECTION";
pub const PRAYER_TIMES_MYSQL_CONNECTION: &'static str = "PRAYER_TIMES_CONNECTION";
pub const PRAYER_TIMES_REDIS_CONNECTION: &'static str = "PRAYER_TIMES_CONNECTION";
pub const ASK_IMAM_MYSQL_CONNECTION: &'static str = "ASK_IMAM_CONNECTION";
pub const ASK_IMAM_REDIS_CONNECTION: &'static str = "ASK_IMAM_CONNECTION";
pub const EVENTS_MYSQL_CONNECTION: &'static str = "EVENTS_CONNECTION";
pub const EVENTS_REDIS_CONNECTION: &'static str = "EVENTS_CONNECTION";

#[inline]
pub(super) fn get_connection_string(
    repository_type: RepositoryType,
    repository_mode: RepositoryMode,
) -> &'static str {
    match (repository_type, repository_mode) {
        (RepositoryType::Authentication, RepositoryMode::InMemory(InMemoryDbProvider::Redis)) => {
            AUTHENTICATION_REDIS_CONNECTION
        }
        (RepositoryType::Authentication, RepositoryMode::Normal(NormalDbProvider::MySql)) => {
            AUTHENTICATION_MYSQL_CONNECTION
        }
        (RepositoryType::PrayerTimes, RepositoryMode::InMemory(InMemoryDbProvider::Redis)) => {
            PRAYER_TIMES_REDIS_CONNECTION
        }
        (RepositoryType::PrayerTimes, RepositoryMode::Normal(NormalDbProvider::MySql)) => {
            PRAYER_TIMES_MYSQL_CONNECTION
        }
        (RepositoryType::AskImam, RepositoryMode::InMemory(InMemoryDbProvider::Redis)) => {
            ASK_IMAM_REDIS_CONNECTION
        }
        (RepositoryType::AskImam, RepositoryMode::Normal(NormalDbProvider::MySql)) => {
            ASK_IMAM_MYSQL_CONNECTION
        }
        (RepositoryType::Events, RepositoryMode::InMemory(InMemoryDbProvider::Redis)) => {
            EVENTS_REDIS_CONNECTION
        }
        (RepositoryType::Events, RepositoryMode::Normal(NormalDbProvider::MySql)) => {
            EVENTS_MYSQL_CONNECTION
        }
    }
}
