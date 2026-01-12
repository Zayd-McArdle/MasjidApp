use async_trait::async_trait;
use masjid_app_api_library::features::prayer_times::errors::GetPrayerTimesError;
use masjid_app_api_library::features::prayer_times::models::PrayerTimesDTO;
use masjid_app_api_library::features::prayer_times::repositories::PrayerTimesRepository;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::data_access::repository_manager::{
    InMemoryRepository, MySqlRepository, RepositoryMode, RepositoryType,
};
use sqlx::mysql::MySqlRow;
use sqlx::{Error, Row};
use std::sync::Arc;

#[async_trait]
pub trait PrayerTimesPublicRepository: PrayerTimesRepository {
    async fn get_updated_prayer_times(
        &self,
        hash: &str,
    ) -> Result<PrayerTimesDTO, GetPrayerTimesError>;
}

pub async fn new_prayer_times_public_repository(
    repository_mode: RepositoryMode,
) -> Arc<dyn PrayerTimesPublicRepository> {
    new_repository!(repository_mode, RepositoryType::PrayerTimes)
}

#[async_trait]
impl PrayerTimesPublicRepository for InMemoryRepository {
    async fn get_updated_prayer_times(
        &self,
        hash: &str,
    ) -> Result<PrayerTimesDTO, GetPrayerTimesError> {
        tracing::warn!("In-memory database for getting updated prayer times not implemented");
        Err(GetPrayerTimesError::UnableToGetPrayerTimes)
    }
}

#[async_trait]
impl PrayerTimesPublicRepository for MySqlRepository {
    async fn get_updated_prayer_times(
        &self,
        hash: &str,
    ) -> Result<PrayerTimesDTO, GetPrayerTimesError> {
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
                Err(GetPrayerTimesError::PrayerTimesNotFound)
            }
            Err(err) => {
                tracing::error!(
                    "unable to get updated prayer times from the database: {}",
                    err
                );
                Err(GetPrayerTimesError::UnableToGetPrayerTimes)
            }
        }
    }
}
