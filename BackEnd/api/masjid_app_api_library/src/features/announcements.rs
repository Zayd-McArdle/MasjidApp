use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mockall::automock;
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

#[derive(Clone, Debug, PartialEq)]
pub enum GetAnnouncementsError {
    //Used for when a database returns no rows.
    AnnouncementsNotFound,
    //Used for when there is some database operational failure
    UnableToFetchAnnouncements,
}

#[automock]
#[async_trait]
pub trait AnnouncementRepository: Send + Sync {
    async fn get_announcements(&self) -> Result<Vec<AnnouncementDTO>, GetAnnouncementsError>;
}
