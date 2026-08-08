use crate::features::prayer_times::errors::update_prayer_times_repository_error::UpdatePrayerTimesRepositoryError;
use crate::features::prayer_times::repositories::PrayerTimesAdminRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;

#[async_trait]
impl PrayerTimesAdminRepository for MySqlRepository {
    async fn update_prayer_times(
        &self,
        prayer_times_data: &PrayerTimesDTO,
    ) -> Result<(), UpdatePrayerTimesRepositoryError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL upsert_prayer_times(?, ?);")
            .bind(&prayer_times_data.data)
            .bind(&prayer_times_data.hash)
            .execute(&*db_connection)
            .await;
        match query_response {
            Ok(_) => {
                tracing::info!("successfully updated prayer times");
                Ok(())
            }
            Err(err) => {
                tracing::error!("unable to update prayer times: {}", err);
                Err(UpdatePrayerTimesRepositoryError::UnableToUpdatePrayerTimes)
            }
        }
    }
}
