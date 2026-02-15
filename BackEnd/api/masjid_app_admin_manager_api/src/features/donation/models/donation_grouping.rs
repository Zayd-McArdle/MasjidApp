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
