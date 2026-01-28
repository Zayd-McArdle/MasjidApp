use masjid_app_api_library::features::donation::models::{Address, StandingOrder};
use masjid_app_api_library::shared::types::payment_details::PaymentDetails;
use serde::Deserialize;
use sqlx::types::chrono::{DateTime, Utc};
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct SendDonationRequest {
    #[serde(rename = "paymentDetails")]
    #[validate(nested)]
    pub payment_details: PaymentDetails,

    #[validate(custom(function = "validate_amount"))]
    pub amount: f64,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 10, max = 10))]
    pub phone_number: String,
}

fn validate_amount(amount: f64) -> Result<(), ValidationError> {
    if amount < 0.01 {
        return Err(ValidationError::new("amount cannot be less than 0.01"));
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
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
