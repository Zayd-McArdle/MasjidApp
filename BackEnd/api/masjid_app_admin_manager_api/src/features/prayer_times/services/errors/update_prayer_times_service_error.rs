use crate::features::prayer_times::errors::UpdatePrayerTimesRepositoryError;

pub enum UpdatePrayerTimesServiceError {
    RepositoryError(UpdatePrayerTimesRepositoryError),
}
impl From<UpdatePrayerTimesRepositoryError> for UpdatePrayerTimesServiceError {
    #[inline]
    fn from(value: UpdatePrayerTimesRepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
