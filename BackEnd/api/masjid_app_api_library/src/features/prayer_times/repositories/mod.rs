use crate::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;
use crate::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use async_trait::async_trait;
use mockall::automock;
mod mysql_impl;
mod redis_impl;
#[automock]
#[async_trait]
pub trait PrayerTimesRepository: Send + Sync {
    async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesRepositoryError>;
}
