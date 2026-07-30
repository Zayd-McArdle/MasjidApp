use crate::features::prayer_times::repositories::PrayerTimesPublicRepository;
use crate::features::prayer_times::services::errors::check_for_updated_prayer_times_error::CheckForUpdatedPrayerTimesError;
use async_trait::async_trait;
use masjid_app_api_library::features::prayer_times::models::PrayerTimesDTO;
use masjid_app_api_library::features::prayer_times::services::service_impl::PrayerTimesServiceImpl;
use masjid_app_api_library::new_prayer_times_service;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait PrayerTimesUpdateCheckingService: Send + Sync {
    async fn check_for_updated_prayer_times(
        &self,
        prayer_times_hash: &str,
    ) -> Result<PrayerTimesDTO, CheckForUpdatedPrayerTimesError>;
}

new_prayer_times_service!(
    new_prayer_times_update_checking_service,
    PrayerTimesUpdateCheckingService,
    PrayerTimesPublicRepository
);

#[async_trait]
impl PrayerTimesUpdateCheckingService for PrayerTimesServiceImpl<dyn PrayerTimesPublicRepository> {
    async fn check_for_updated_prayer_times(
        &self,
        prayer_times_hash: &str,
    ) -> Result<PrayerTimesDTO, CheckForUpdatedPrayerTimesError> {
        if let Ok(updated_prayer_times) = self
            .common
            .in_memory_repository
            .get_updated_prayer_times(prayer_times_hash)
            .await
        {
            return Ok(updated_prayer_times);
        }
        self.common
            .repository
            .get_updated_prayer_times(prayer_times_hash)
            .await
            .map_err(CheckForUpdatedPrayerTimesError::from)
    }
}
