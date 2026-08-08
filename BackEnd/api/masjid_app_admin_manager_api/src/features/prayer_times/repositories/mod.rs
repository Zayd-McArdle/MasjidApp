use crate::features::prayer_times::errors::update_prayer_times_repository_error::UpdatePrayerTimesRepositoryError;
use async_trait::async_trait;
use masjid_app_api_library::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use masjid_app_api_library::features::prayer_times::repositories::PrayerTimesRepository;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use masjid_app_api_library::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use masjid_app_api_library::shared::data_access::repository_management::repository_mode::RepositoryMode;
use masjid_app_api_library::shared::data_access::repository_management::repository_type::RepositoryType;
use std::sync::Arc;

mod mysql_impl;
mod redis_impl;

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
