use crate::features::donation::models::donation_filter::DonationFilter;
use crate::features::donation::models::donation_grouping::DonationGrouping;

pub struct DonationFilterWithGrouping {
    pub filter: DonationFilter,
    pub donation_grouping: DonationGrouping,
}
