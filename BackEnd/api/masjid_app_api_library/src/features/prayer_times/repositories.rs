use async_trait::async_trait;
use mockall::automock;
use sqlx::mysql::MySqlRow;
use sqlx::{Error, Row};
use crate::features::prayer_times::errors::GetPrayerTimesError;
use crate::features::prayer_times::models::PrayerTimesDTO;
use crate::shared::data_access::repository_manager::{InMemoryRepository, MySqlRepository};

#[automock]
#[async_trait]
pub trait PrayerTimesRepository: Send + Sync {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError>;
}

#[async_trait]
impl PrayerTimesRepository for InMemoryRepository {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError> {
        tracing::warn!("In-memory database for getting prayer times not implemented");
        Err(GetPrayerTimesError::UnableToGetPrayerTimes)
    }
}

#[async_trait]
impl PrayerTimesRepository for MySqlRepository {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL get_prayer_times();")
            .fetch_one(&*db_connection)
            .await
            .map(|row: MySqlRow| PrayerTimesDTO {
                data: row.get(0),
                hash: row.get(1),
            });

        match query_response {
            Ok(prayer_times) => Ok(prayer_times),
            Err(Error::RowNotFound) => Err(GetPrayerTimesError::PrayerTimesNotFound),
            Err(err) => {
                tracing::error!("unable to get prayer times from the database: {}", err);
                Err(GetPrayerTimesError::UnableToGetPrayerTimes)
            }
        }
    }
}