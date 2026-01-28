use masjid_app_api_library::features::donation::models::DonationDetails;
use masjid_app_api_library::shared::payment::billing_address::BillingAddress;
use masjid_app_api_library::shared::payment::card_details::CardDetails;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
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
}
