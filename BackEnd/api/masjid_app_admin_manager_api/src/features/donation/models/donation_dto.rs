use masjid_app_api_library::features::donation::models::donation_details::DonationDetails;
use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;
use masjid_app_api_library::shared::payment::billing_address::BillingAddress;
use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
use masjid_app_api_library::shared::types::contact_details::ContactDetails;
use masjid_app_api_library::shared::types::personal_title::PersonalTitle;
use masjid_app_api_library::shared::types::recurrence::Recurrence;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DonationHistoryDTO {
    pub id: i64,
    #[serde(rename = "donationDetails")]
    pub donation_details: DonationDetails,
    #[serde(rename = "donorAddress")]
    pub donor_address: BillingAddress,
    #[serde(rename = "transactionStatus")]
    pub transaction_status: TransactionStatus,
}

impl From<DonationHistory> for DonationHistoryDTO {
    fn from(donation_history: DonationHistory) -> Self {
        Self {
            id: donation_history.id,
            donation_details: DonationDetails {
                cause: donation_history.cause,
                donation_intention: DonationIntention::from_str(
                    &donation_history.donation_intention,
                )
                .unwrap(),
                is_gift_aid: donation_history.is_gift_aid,
                contact_details: ContactDetails {
                    full_name: donation_history.donor_full_name,
                    title: PersonalTitle::from_str(&donation_history.donor_title).ok(),
                    phone_number: donation_history.phone_number,
                    email: donation_history.email,
                },
                amount: donation_history.amount,
                donation_frequency: Recurrence::from_str(&donation_history.donation_frequency)
                    .unwrap(),
            },
            donor_address: BillingAddress {
                line_1: donation_history.address_line_1,
                line_2: donation_history.address_line_2,
                city: donation_history.address_city,
                region: donation_history.address_region,
                country: donation_history.address_country,
                postal_code: donation_history.address_postal,
            },
            transaction_status: TransactionStatus::from_str(&donation_history.transaction_status)
                .unwrap(),
        }
    }
}
