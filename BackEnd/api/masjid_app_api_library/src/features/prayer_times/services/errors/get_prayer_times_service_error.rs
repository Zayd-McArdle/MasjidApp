use crate::features::prayer_times::errors::GetPrayerTimesRepositoryError;

pub enum GetPrayerTimesServiceError {
    RepositoryError(GetPrayerTimesRepositoryError),
}

impl From<GetPrayerTimesRepositoryError> for GetPrayerTimesServiceError {
    fn from(value: GetPrayerTimesRepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
