use crate::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;
use crate::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use crate::features::prayer_times::repositories::PrayerTimesRepository;
use crate::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use async_trait::async_trait;
use sqlx::mysql::MySqlRow;
use sqlx::{Error, Row};

#[async_trait]
impl PrayerTimesRepository for MySqlRepository {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesRepositoryError> {
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
            Err(Error::RowNotFound) => Err(GetPrayerTimesRepositoryError::PrayerTimesNotFound),
            Err(err) => {
                tracing::error!("unable to get prayer times from the database: {}", err);
                Err(GetPrayerTimesRepositoryError::UnableToGetPrayerTimes)
            }
        }
    }
}
