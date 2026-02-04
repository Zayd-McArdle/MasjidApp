use crate::shared::types::age_range::AgeRange;
use crate::shared::types::contact_details::ContactDetails;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, PartialEq, Eq)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Confirmed,
    Cancelled,
}
impl ToString for EventStatus {
    fn to_string(&self) -> String {
        match self {
            EventStatus::Confirmed => "confirmed".to_owned(),
            EventStatus::Cancelled => "cancelled".to_owned(),
        }
    }
}
impl FromStr for EventStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "confirmed" => Ok(EventStatus::Confirmed),
            "cancelled" => Ok(EventStatus::Cancelled),
            _ => Err(()),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::Type, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Talk,
    Social,
    Class,
}
impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            EventType::Talk => "talk".to_owned(),
            EventType::Social => "social".to_owned(),
            EventType::Class => "class".to_owned(),
        }
    }
}

impl FromStr for EventType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "talk" => Ok(EventType::Talk),
            "social" => Ok(EventType::Social),
            "class" => Ok(EventType::Class),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
pub enum EventRecurrence {
    OneOff,
    Daily,
    Weekly,
    Fortnightly,
    Monthly,
}

impl ToString for EventRecurrence {
    fn to_string(&self) -> String {
        match self {
            EventRecurrence::OneOff => "one-off".to_owned(),
            EventRecurrence::Daily => "daily".to_owned(),
            EventRecurrence::Weekly => "weekly".to_owned(),
            EventRecurrence::Fortnightly => "fortnightly".to_owned(),
            EventRecurrence::Monthly => "monthly".to_owned(),
        }
    }
}

impl FromStr for EventRecurrence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one-off" => Ok(EventRecurrence::OneOff),
            "daily" => Ok(EventRecurrence::Daily),
            "weekly" => Ok(EventRecurrence::Weekly),
            "fortnight" => Ok(EventRecurrence::Fortnightly),
            "monthly" => Ok(EventRecurrence::Monthly),
            _ => Err(()),
        }
    }
}

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
#[derive(sqlx::FromRow, Clone, Debug, PartialEq)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    // Event Details
    pub r#type: String,
    pub recurrence: String,
    pub status: String,
    pub minimum_age: Option<u8>,
    pub maximum_age: Option<u8>,
    pub image_url: Option<String>,
    // Organiser Contact Details
    pub full_name: String,
    pub phone_number: String,
    pub email: Option<String>,
}

impl From<EventDTO> for Event {
    fn from(dto: EventDTO) -> Self {
        let (minimum_age, maximum_age): (Option<u8>, Option<u8>) = match dto.event_details.age_range
        {
            None => (None, None),
            Some(age_range) => (Some(age_range.minimum_age), Some(age_range.maximum_age)),
        };
        Self {
            id: dto.id,
            title: dto.title,
            description: dto.description,
            date: dto.date,
            r#type: dto.event_details.event_type.to_string(),
            recurrence: dto.event_details.event_recurrence.to_string(),
            status: dto.event_details.event_status.to_string(),
            minimum_age: minimum_age,
            maximum_age: maximum_age,
            image_url: dto.event_details.image_url,
            full_name: dto.event_details.contact_details.full_name,
            phone_number: dto.event_details.contact_details.phone_number,
            email: dto.event_details.contact_details.email,
        }
    }
}

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
                age_range: age_range,
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
