use crate::features::prayer_times::errors::UpdatePrayerTimesRepositoryError;
use async_trait::async_trait;
use masjid_app_api_library::features::prayer_times::models::PrayerTimesDTO;
use masjid_app_api_library::features::prayer_times::repositories::PrayerTimesRepository;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use masjid_app_api_library::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use masjid_app_api_library::shared::data_access::repository_management::repository_mode::RepositoryMode;
use masjid_app_api_library::shared::data_access::repository_management::repository_type::RepositoryType;
use std::sync::Arc;

#[async_trait]
pub trait PrayerTimesAdminRepository: PrayerTimesRepository {
    async fn update_prayer_times(
        &self,
        prayer_times_data: &PrayerTimesDTO,
    ) -> Result<(), UpdatePrayerTimesRepositoryError>;
}

pub async fn new_prayer_times_admin_repository(
    repository_mode: RepositoryMode,
) -> Arc<dyn PrayerTimesAdminRepository> {
    new_repository!(repository_mode, RepositoryType::PrayerTimes)
}
#[async_trait]
impl PrayerTimesAdminRepository for InMemoryRepository {
    async fn update_prayer_times(
        &self,
        prayer_times_data: &PrayerTimesDTO,
    ) -> Result<(), UpdatePrayerTimesRepositoryError> {
        Err(UpdatePrayerTimesRepositoryError::UnableToUpdatePrayerTimes)
    }
}

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
