use crate::features::prayer_times::models::PrayerTimesDTO;
use crate::features::prayer_times::repositories::PrayerTimesRepository;
use crate::features::prayer_times::services::errors::get_prayer_times_service_error::GetPrayerTimesServiceError;
use crate::features::prayer_times::services::service_impl::PrayerTimesServiceImpl;
use crate::new_prayer_times_service;
use crate::shared::common_service_impl::CommonServiceImpl;
use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait PrayerTimesRetrievalService: Send + Sync {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesServiceError>;
}

new_prayer_times_service!(
    new_prayer_times_retrieval_service,
    PrayerTimesRetrievalService,
    PrayerTimesRepository
);

#[async_trait]
impl PrayerTimesRetrievalService for PrayerTimesServiceImpl<dyn PrayerTimesRepository> {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesServiceError> {
        if let Ok(prayer_times) = self.common.in_memory_repository.get_prayer_times().await {
            return Ok(prayer_times);
        }
        self.common
            .repository
            .get_prayer_times()
            .await
            .map_err(GetPrayerTimesServiceError::from)
    }
}
