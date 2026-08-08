use crate::features::prayer_times::repositories::PrayerTimesPublicRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;
use masjid_app_api_library::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use sqlx::mysql::MySqlRow;
use sqlx::{Error, Row};

#[async_trait]
impl PrayerTimesPublicRepository for MySqlRepository {
    async fn get_updated_prayer_times(
        &self,
        hash: &str,
    ) -> Result<PrayerTimesDTO, GetPrayerTimesRepositoryError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL get_updated_prayer_times(?);")
            .bind(hash)
            .fetch_one(&*db_connection)
            .await
            .map(|row: MySqlRow| {
                if row.len() == 1 {
                    tracing::debug!("prayer times hash matches request hash");
                    return PrayerTimesDTO {
                        data: None,
                        hash: row.get(0),
                    };
                }
                tracing::debug!(
                    "prayer times hash does not match request hash. downloading new prayer times"
                );
                return PrayerTimesDTO {
                    data: row.get(0),
                    hash: row.get(1),
                };
            });
        match query_response {
            Ok(prayer_times) => Ok(prayer_times),
            Err(Error::RowNotFound) => {
                tracing::error!("prayer times not found");
                Err(GetPrayerTimesRepositoryError::PrayerTimesNotFound)
            }
            Err(err) => {
                tracing::error!(
                    "unable to get updated prayer times from the database: {}",
                    err
                );
                Err(GetPrayerTimesRepositoryError::UnableToGetPrayerTimes)
            }
        }
    }
}
