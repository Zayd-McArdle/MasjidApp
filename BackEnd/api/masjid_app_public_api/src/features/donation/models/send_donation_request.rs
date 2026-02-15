use masjid_app_api_library::features::donation::models::donation_details::DonationDetails;
use masjid_app_api_library::shared::services::payment::billing_address::BillingAddress;
use masjid_app_api_library::shared::services::payment::card_details::CardDetails;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct SendDonationRequest {
    #[serde(rename = "donationDetails")]
    #[validate(nested)]
    pub donation_details: DonationDetails,

    #[serde(rename = "cardDetails")]
    #[validate(nested)]
    pub card_details: CardDetails,

    #[serde(rename = "billingAddress")]
    #[validate(nested)]
    pub billing_address: BillingAddress,

    // is_email_requested specifies whether the user wants a receipt of donation emailed to them
    pub is_email_requested: bool,
}
