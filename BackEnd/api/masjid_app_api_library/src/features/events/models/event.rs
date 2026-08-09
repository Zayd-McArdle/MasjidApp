use crate::features::events::models::event_dto::EventDTO;
use chrono::{DateTime, Utc};

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
            minimum_age,
            maximum_age,
            image_url: dto.event_details.image_url,
            full_name: dto.event_details.contact_details.full_name,
            phone_number: dto.event_details.contact_details.phone_number,
            email: dto.event_details.contact_details.email,
        }
    }
}
