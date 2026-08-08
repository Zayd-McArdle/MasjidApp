use crate::features::events::models::event::Event;
use crate::features::events::models::event_details::EventDetails;
use crate::features::events::models::event_recurrence::EventRecurrence;
use crate::features::events::models::event_status::EventStatus;
use crate::features::events::models::event_type::EventType;
use crate::shared::types::age_range::AgeRange;
use crate::shared::types::contact_details::ContactDetails;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Validate)]
pub struct EventDTO {
    pub id: i32,

    #[validate(length(min = 4))]
    pub title: String,

    #[validate(length(min = 4))]
    pub description: Option<String>,

    pub date: DateTime<Utc>,

    #[validate(nested)]
    #[serde(rename(serialize = "eventDetails", deserialize = "eventDetails"))]
    pub event_details: EventDetails,
}

impl From<Event> for EventDTO {
    fn from(event: Event) -> Self {
        let mut age_range: Option<AgeRange> = None;
        if event.minimum_age.is_some() && event.maximum_age.is_some() {
            age_range = Some(AgeRange {
                minimum_age: event.minimum_age.unwrap(),
                maximum_age: event.maximum_age.unwrap(),
            });
        }
        Self {
            id: event.id,
            title: event.title,
            description: event.description,
            date: event.date,
            event_details: EventDetails {
                event_type: EventType::from_str(&event.r#type).unwrap(),
                event_recurrence: EventRecurrence::from_str(&event.recurrence).unwrap(),
                event_status: EventStatus::from_str(&event.status).unwrap(),
                age_range,
                image_url: event.image_url,
                contact_details: ContactDetails {
                    full_name: event.full_name,
                    title: None,
                    phone_number: event.phone_number,
                    email: event.email,
                },
            },
        }
    }
}
