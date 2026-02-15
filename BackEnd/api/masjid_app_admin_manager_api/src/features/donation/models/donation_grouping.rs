use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum DonationGrouping {
    Cause,
    Intention,
    Amount,
    GiftAid,
    Frequency,
    Status,
}
impl Default for DonationGrouping {
    fn default() -> Self {
        Self::Cause
    }
}
