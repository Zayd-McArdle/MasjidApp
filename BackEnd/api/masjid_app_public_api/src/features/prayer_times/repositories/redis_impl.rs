use crate::features::prayer_times::repositories::PrayerTimesPublicRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;
use masjid_app_api_library::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;

#[async_trait]
impl PrayerTimesPublicRepository for InMemoryRepository {
    async fn get_updated_prayer_times(
        &self,
        hash: &str,
    ) -> Result<PrayerTimesDTO, GetPrayerTimesRepositoryError> {
        tracing::warn!("In-memory database for getting updated prayer times not implemented");
        Err(GetPrayerTimesRepositoryError::UnableToGetPrayerTimes)
    }
}
