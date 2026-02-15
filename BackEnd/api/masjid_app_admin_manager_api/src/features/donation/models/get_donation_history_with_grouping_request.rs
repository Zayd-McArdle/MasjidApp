use crate::features::donation::models::donation_filter::DonationFilter;
use crate::features::donation::models::donation_filter_with_grouping::DonationFilterWithGrouping;
use crate::features::donation::models::donation_grouping::DonationGrouping;
use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;
use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
use masjid_app_api_library::shared::types::recurrence::Recurrence;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct GetDonationHistoryWithGroupingRequest {
    #[validate(length(min = 3, max = 25))]
    pub donation_cause: Option<String>,
    pub donation_intention: Option<DonationIntention>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 10, max = 10))]
    pub phone_number: Option<String>,
    pub amount: Option<u32>,
    pub is_gift_aid: Option<bool>,
    pub donation_frequency: Option<Recurrence>,
    pub transaction_status: Option<TransactionStatus>,
    pub donation_grouping: DonationGrouping,
}

impl TryInto<DonationFilterWithGrouping> for GetDonationHistoryWithGroupingRequest {
    type Error = String;

    fn try_into(self) -> Result<DonationFilterWithGrouping, Self::Error> {
        self.validate().map_err(|err| err.to_string())?;
        Ok(DonationFilterWithGrouping {
            filter: DonationFilter {
                donation_cause: self.donation_cause,
                donation_intention: self.donation_intention,
                email: self.email,
                phone_number: self.phone_number,
                amount: self.amount,
                is_gift_aid: self.is_gift_aid,
                donation_frequency: self.donation_frequency,
                transaction_status: self.transaction_status,
            },
            donation_grouping: self.donation_grouping,
        })
    }
}
