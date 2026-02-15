use masjid_app_api_library::shared::traits::value_retriever::ValueRetriever;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum DonationGrouping {
    Cause,
    Intention,
    Amount,
    GiftAid,
    Frequency,
    TransactionStatus,
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

impl ValueRetriever for DonationGrouping {
    fn get_values() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            DonationGrouping::Cause,
            DonationGrouping::Intention,
            DonationGrouping::Amount,
            DonationGrouping::GiftAid,
            DonationGrouping::Frequency,
            DonationGrouping::TransactionStatus,
        ]
    }
}
