use crate::shared::payment::billing::address::Address;
use crate::shared::types::recurrence::Recurrence;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub enum DonationMethod {
    Now,
    Deferred,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct StandingOrder {
    account_holder: String,
    bank_name: String,
    account_number: String,
    sort_code: String,
    frequency: Recurrence,
    amount_per_donation: Option<u16>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
}
pub struct DonationDTO {
    pub id: u64,
    pub cause: String,
    pub is_gift_aid: bool,
    pub title: String,
    pub donor_name: String,
    pub address: Address,
    pub phone_number: String,
    pub amount: u16,
    pub standing_order: Option<StandingOrder>,
    pub date_donated: DateTime<Utc>,
}
