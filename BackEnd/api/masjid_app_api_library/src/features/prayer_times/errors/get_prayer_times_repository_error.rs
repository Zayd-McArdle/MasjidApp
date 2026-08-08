#[derive(Clone, Debug, PartialEq)]
pub enum GetPrayerTimesRepositoryError {
    PrayerTimesNotFound,
    UnableToGetPrayerTimes,
}
