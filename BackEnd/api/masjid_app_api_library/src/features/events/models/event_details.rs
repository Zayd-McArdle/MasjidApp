use crate::features::events::models::event_recurrence::EventRecurrence;
use crate::features::events::models::event_status::EventStatus;
use crate::features::events::models::event_type::EventType;
use crate::shared::types::age_range::AgeRange;
use crate::shared::types::contact_details::ContactDetails;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Validate)]
pub struct EventDetails {
    #[serde(rename(serialize = "eventType", deserialize = "eventType"))]
    pub event_type: EventType,

    #[serde(rename(serialize = "eventRecurrence", deserialize = "eventRecurrence"))]
    pub event_recurrence: EventRecurrence,

    #[serde(rename(serialize = "eventStatus", deserialize = "eventStatus"))]
    pub event_status: EventStatus,

    #[validate(nested)]
    #[serde(rename(serialize = "ageRange", deserialize = "ageRange"))]
    pub age_range: Option<AgeRange>,

    #[validate(url)]
    #[serde(rename(serialize = "imageUrl", deserialize = "imageUrl"))]
    pub image_url: Option<String>,

    #[validate(nested)]
    #[serde(rename(serialize = "contactDetails", deserialize = "contactDetails"))]
    pub contact_details: ContactDetails,
}
