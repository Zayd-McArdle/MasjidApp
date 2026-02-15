use crate::features::donation::models::donation_details::DonationDetails;
use crate::shared::payment::billing_address::BillingAddress;
use crate::shared::payment::transaction_status::TransactionStatus;

#[derive(Debug, Clone)]
pub struct DonationHistory {
    pub id: i64,
    pub cause: String,
    pub donation_intention: String,
    pub donor_full_name: String,
    pub donor_title: String,
    pub phone_number: String,
    pub email: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub address_city: String,
    pub address_region: String,
    pub address_country: Option<String>,
    pub address_postal: String,
    pub amount: u32,
    pub is_gift_aid: bool,
    pub donation_frequency: String,
    pub transaction_status: String,
}
impl DonationHistory {
    pub fn new(
        donation_details: DonationDetails,
        address: BillingAddress,
        transaction_status: TransactionStatus,
    ) -> Self {
        DonationHistory {
            id: 0,
            cause: donation_details.cause,
            donation_intention: donation_details.donation_intention.to_string(),
            donor_full_name: donation_details.contact_details.full_name,
            donor_title: donation_details.contact_details.title.unwrap().to_string(),
            phone_number: donation_details.contact_details.phone_number,
            email: donation_details.contact_details.email,
            address_line_1: address.line_1,
            address_line_2: address.line_2,
            address_city: address.city,
            address_region: address.region,
            address_country: address.country,
            address_postal: address.postal_code,
            amount: donation_details.amount,
            is_gift_aid: donation_details.is_gift_aid,
            donation_frequency: donation_details.donation_frequency.to_string(),
            transaction_status: transaction_status.to_string(),
        }
    }
}
