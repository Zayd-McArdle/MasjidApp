use crate::features::prayer_times::errors::get_prayer_times_repository_error::GetPrayerTimesRepositoryError;

pub enum GetPrayerTimesServiceError {
    RepositoryError(GetPrayerTimesRepositoryError),
}

impl From<GetPrayerTimesRepositoryError> for GetPrayerTimesServiceError {
    #[inline]
    fn from(value: GetPrayerTimesRepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
