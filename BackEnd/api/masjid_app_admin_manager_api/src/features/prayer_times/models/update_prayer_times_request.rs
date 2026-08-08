use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Clone, Validate)]
pub struct UpdatePrayerTimesRequest {
    #[serde(rename = "prayerTimesData")]
    pub prayer_times_data: Vec<u8>,
    #[validate(length(equal = 64))]
    pub hash: String,
}
