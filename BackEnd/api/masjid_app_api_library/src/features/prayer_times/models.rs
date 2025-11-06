use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PrayerTimesDTO {
    pub data: Option<Vec<u8>>,
    pub hash: String,
}