use crate::features::prayer_times::repositories::PrayerTimesRepository;
use crate::shared::common_service_impl::CommonServiceImpl;

pub struct PrayerTimesServiceImpl<R: PrayerTimesRepository + ?Sized> {
    pub common: CommonServiceImpl<R>,
}
