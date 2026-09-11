use crate::features::prayer_times::errors::update_prayer_times_repository_error::UpdatePrayerTimesRepositoryError;

#[derive(Debug)]
pub enum UpdatePrayerTimesServiceError {
    RepositoryError(UpdatePrayerTimesRepositoryError),
}
impl From<UpdatePrayerTimesRepositoryError> for UpdatePrayerTimesServiceError {
    #[inline]
    fn from(value: UpdatePrayerTimesRepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
