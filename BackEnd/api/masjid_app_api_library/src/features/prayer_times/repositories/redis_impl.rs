use crate::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;
use crate::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use crate::features::prayer_times::repositories::PrayerTimesRepository;
use crate::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use async_trait::async_trait;

#[async_trait]
impl PrayerTimesRepository for InMemoryRepository {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesRepositoryError> {
        tracing::warn!("In-memory database for getting prayer times not implemented");
        Err(GetPrayerTimesRepositoryError::UnableToGetPrayerTimes)
    }
}
