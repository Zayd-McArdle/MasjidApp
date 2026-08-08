use crate::features::prayer_times::errors::update_prayer_times_repository_error::UpdatePrayerTimesRepositoryError;
use crate::features::prayer_times::repositories::PrayerTimesAdminRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;

#[async_trait]
impl PrayerTimesAdminRepository for InMemoryRepository {
    async fn update_prayer_times(
        &self,
        prayer_times_data: &PrayerTimesDTO,
    ) -> Result<(), UpdatePrayerTimesRepositoryError> {
        Err(UpdatePrayerTimesRepositoryError::UnableToUpdatePrayerTimes)
    }
}
