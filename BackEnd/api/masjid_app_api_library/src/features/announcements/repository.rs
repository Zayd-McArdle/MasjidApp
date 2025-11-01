use crate::features::announcements::errors::GetAnnouncementsError;
use crate::features::announcements::models::AnnouncementDTO;
use async_trait::async_trait;
use mockall::automock;
#[automock]
#[async_trait]
pub trait AnnouncementRepository: Send + Sync {
    async fn get_announcements(&self) -> Result<Vec<AnnouncementDTO>, GetAnnouncementsError>;
}
