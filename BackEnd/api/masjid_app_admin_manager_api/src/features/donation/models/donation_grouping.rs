use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize)]
pub enum DonationGrouping {
    Cause,
    Intention,
    Amount,
    GiftAid,
    Frequency,
    TransactionStatus,
}
impl Default for DonationGrouping {
    fn default() -> Self {
        Self::Cause
    }
}

impl Display for DonationGrouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const CAUSE: &'static str = "cause";
        const INTENTION: &'static str = "intention";
        const AMOUNT: &'static str = "amount";
        const GIFT_AID: &'static str = "gift_aid";
        const FREQUENCY: &'static str = "frequency";
        const TRANSACTION_STATUS: &'static str = "transaction_status";
        let str = match self {
            DonationGrouping::Cause => CAUSE,
            DonationGrouping::Intention => INTENTION,
            DonationGrouping::Amount => AMOUNT,
            DonationGrouping::GiftAid => GIFT_AID,
            DonationGrouping::Frequency => FREQUENCY,
            DonationGrouping::TransactionStatus => TRANSACTION_STATUS,
        }
        .to_string();
        write!(f, "{}", str)
    }
}
