use crate::features::prayer_times::repositories::PrayerTimesAdminRepository;
use crate::features::prayer_times::services::errors::update_prayer_times_service_error::UpdatePrayerTimesServiceError;
use async_trait::async_trait;
use masjid_app_api_library::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use masjid_app_api_library::features::prayer_times::services::service_impl::PrayerTimesServiceImpl;
use masjid_app_api_library::new_prayer_times_service;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait PrayerTimesUpdateService: Send + Sync {
    async fn update_prayer_times(
        &self,
        prayer_times: PrayerTimesDTO,
    ) -> Result<(), UpdatePrayerTimesServiceError>;
}

new_prayer_times_service!(
    new_prayer_times_update_service,
    PrayerTimesUpdateService,
    PrayerTimesAdminRepository
);

#[async_trait]
impl PrayerTimesUpdateService for PrayerTimesServiceImpl<dyn PrayerTimesAdminRepository> {
    async fn update_prayer_times(
        &self,
        prayer_times: PrayerTimesDTO,
    ) -> Result<(), UpdatePrayerTimesServiceError> {
        if let Err(update_prayer_times_error) = self
            .common
            .in_memory_repository
            .update_prayer_times(&prayer_times)
            .await
        {
            tracing::warn!(
                ?update_prayer_times_error,
                "unable to update prayer times to in-memory repository"
            );
        }
        self.common
            .repository
            .update_prayer_times(&prayer_times)
            .await
            .map_err(UpdatePrayerTimesServiceError::RepositoryError)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::prayer_times::errors::update_prayer_times_repository_error::UpdatePrayerTimesRepositoryError;
    use masjid_app_api_library::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;
    use masjid_app_api_library::features::prayer_times::repositories::PrayerTimesRepository;
    use mockall::mock;

    mock! {
        pub PrayerTimesAdminRepository {}

        // Implement the base trait
        #[async_trait]
        impl PrayerTimesRepository for PrayerTimesAdminRepository {
            async fn get_prayer_times(&self) -> Result<PrayerTimesDTO, GetPrayerTimesRepositoryError>;
        }

        // Implement the admin trait
        #[async_trait]
        impl PrayerTimesAdminRepository for PrayerTimesAdminRepository {
            async fn update_prayer_times(&self, prayer_times_data: &PrayerTimesDTO) -> Result<(), UpdatePrayerTimesRepositoryError>;
        }
    }
}
