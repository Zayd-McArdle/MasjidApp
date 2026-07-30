use crate::features::prayer_times::errors::UpdatePrayerTimesRepositoryError;

pub enum UpdatePrayerTimesServiceError {
    RepositoryError(UpdatePrayerTimesRepositoryError),
}
impl From<UpdatePrayerTimesRepositoryError> for UpdatePrayerTimesServiceError {
    fn from(value: UpdatePrayerTimesRepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
