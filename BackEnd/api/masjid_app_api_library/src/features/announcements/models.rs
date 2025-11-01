use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AnnouncementDTO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub image: Option<Vec<u8>>,
    pub author: String,
}