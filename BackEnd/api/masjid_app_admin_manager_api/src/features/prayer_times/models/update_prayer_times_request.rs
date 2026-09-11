use serde::Deserialize;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePrayerTimesRequest {
    #[serde(rename = "prayerTimesData")]
    pub prayer_times_data: Vec<u8>,
    pub hash: String,
}

impl Validate for UpdatePrayerTimesRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if self.prayer_times_data.is_empty() {
            errors.add(
                "prayer_times_data",
                ValidationError::new("prayer times cannot be empty"),
            );
        }
        if self.hash.len() != 64 {
            errors.add(
                "hash",
                ValidationError::new("hash must be 64 characters long"),
            );
        } else if !self.hash.chars().all(|c| c.is_ascii_hexdigit()) {
            errors.add(
                "hash",
                ValidationError::new("hash must be hexadecimal characters"),
            );
        }
        if errors.is_empty() {
            return Ok(());
        }
        Err(errors)
    }
}
