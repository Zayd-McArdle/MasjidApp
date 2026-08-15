#[derive(Debug)]
pub enum GetPrayerTimesRepositoryError {
    PrayerTimesNotFound,
    UnableToGetPrayerTimes,
}
