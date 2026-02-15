use crate::features::donation::models::donation_intention::DonationIntention;
use crate::shared::types::contact_details::ContactDetails;
use crate::shared::types::recurrence::Recurrence;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone, Default, PartialEq)]
pub struct DonationDetails {
    #[validate(length(min = 3))]
    pub cause: String,

    #[serde(rename = "donationIntention")]
    pub donation_intention: DonationIntention,

    #[serde(rename = "isGiftAid")]
    pub is_gift_aid: bool,
    #[serde(rename = "contactDetails")]
    #[validate(nested)]
    pub contact_details: ContactDetails,
    pub amount: u32,

    #[serde(rename = "donationFrequency")]
    pub donation_frequency: Recurrence,
}
