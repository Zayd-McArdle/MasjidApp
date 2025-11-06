#[derive(Clone, Debug, PartialEq)]
pub enum GetPrayerTimesError {
    PrayerTimesNotFound,
    UnableToGetPrayerTimes,
}