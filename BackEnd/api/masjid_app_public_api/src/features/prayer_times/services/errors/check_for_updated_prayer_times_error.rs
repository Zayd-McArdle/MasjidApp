use masjid_app_api_library::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;

pub enum CheckForUpdatedPrayerTimesError {
    RepositoryError(GetPrayerTimesRepositoryError),
}
impl From<GetPrayerTimesRepositoryError> for CheckForUpdatedPrayerTimesError {
    #[inline]
    fn from(value: GetPrayerTimesRepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
