use std::sync::Arc;
use crate::features::announcements::AnnouncementRepository;
use crate::features::prayer_times::PrayerTimesRepository;
use crate::features::user_authentication::UserRepository;

#[derive(Default)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub announcement_repository: Arc<dyn AnnouncementRepository>,
    pub prayer_times_repository: Arc<dyn PrayerTimesRepository>
}