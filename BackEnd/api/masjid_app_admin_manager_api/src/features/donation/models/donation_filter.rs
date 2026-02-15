use crate::features::donation::models::donation_grouping::DonationGrouping;
use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;
use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
use masjid_app_api_library::shared::types::recurrence::Recurrence;

#[derive(Default)]
pub struct DonationFilter {
    pub donation_cause: Option<String>,
    pub donation_intention: Option<DonationIntention>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub amount: Option<u32>,
    pub is_gift_aid: Option<bool>,
    pub donation_frequency: Option<Recurrence>,
    pub transaction_status: Option<TransactionStatus>,
    pub donation_grouping: Option<DonationGrouping>,
}
