use crate::shared::payment::billing_address::BillingAddress;
use crate::shared::types::contact_details::ContactDetails;
use crate::shared::types::recurrence::Recurrence;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

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
    pub address: BillingAddress,
    pub phone_number: String,
    pub amount: u16,
    pub standing_order: Option<StandingOrder>,
    pub date_donated: DateTime<Utc>,
}
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DonationDetails {
    #[validate(length(min = 3))]
    pub cause: String,
    #[serde(rename = "isGiftAid")]
    pub is_gift_aid: bool,
    #[serde(rename = "contactDetails")]
    #[validate(nested)]
    pub contact_details: ContactDetails,
    #[validate(custom(function = "validate_amount"))]
    pub amount: f64,

    #[serde(rename = "donationFrequency")]
    pub donation_frequency: Recurrence,
}

fn validate_amount(amount: f64) -> Result<(), ValidationError> {
    if amount < 0.01 {
        return Err(ValidationError::new("amount cannot be less than 0.01"));
    }
    Ok(())
}
