use std::sync::Arc;
use crate::features::announcements::AnnouncementRepository;
use crate::features::prayer_times::PrayerTimesRepository;
use crate::features::user_authentication::UserRepository;

#[derive(Default, Clone)]
pub struct InnerAppState<T> {
    //This field allows you to write data to any database
    pub repositories: Vec<T>,
}
#[derive(Clone)]
pub struct OuterAppState {
    pub user_authentication_state: InnerAppState<Arc<dyn UserRepository>>,
    pub prayer_times_state: InnerAppState<Arc<dyn PrayerTimesRepository>>,
    pub announcements_state: InnerAppState<Arc<dyn AnnouncementRepository>>,
}