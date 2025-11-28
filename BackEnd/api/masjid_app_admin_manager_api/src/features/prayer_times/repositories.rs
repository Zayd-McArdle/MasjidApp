use crate::features::prayer_times::errors::UpdatePrayerTimesError;
use async_trait::async_trait;
use masjid_app_api_library::features::prayer_times::models::PrayerTimesDTO;
use masjid_app_api_library::features::prayer_times::repositories::PrayerTimesRepository;
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::data_access::repository_manager::{
    MySqlRepository, RepositoryType,
};
use std::sync::Arc;

#[async_trait]
pub trait PrayerTimesAdminRepository: PrayerTimesRepository {
    async fn update_prayer_times(
        &self,
        prayer_times_data: PrayerTimesDTO,
    ) -> Result<(), UpdatePrayerTimesError>;
}

pub async fn new_prayer_times_admin_repository(
    db_type: DbType,
) -> Arc<dyn PrayerTimesAdminRepository> {
    Arc::new(MySqlRepository::new(RepositoryType::PrayerTimes).await)
}
#[async_trait]
impl PrayerTimesAdminRepository for MySqlRepository {
    async fn update_prayer_times(
        &self,
        prayer_times_data: PrayerTimesDTO,
    ) -> Result<(), UpdatePrayerTimesError> {
        let db_connection = self.db_connection.clone();
        let query_response = sqlx::query("CALL upsert_prayer_times(?, ?);")
            .bind(prayer_times_data.data)
            .bind(prayer_times_data.hash)
            .execute(&*db_connection)
            .await;
        match query_response {
            Ok(_) => {
                tracing::info!("successfully updated prayer times");
                Ok(())
            }
            Err(err) => {
                tracing::error!("unable to update prayer times: {}", err);
                Err(UpdatePrayerTimesError::UnableToUpdatePrayerTimes)
            }
        }
    }
}
