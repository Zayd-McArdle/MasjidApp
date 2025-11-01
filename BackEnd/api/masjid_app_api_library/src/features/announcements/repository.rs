use mockall::automock;
use crate::features::announcements::models::AnnouncementDTO;
use crate::features::announcements::errors::GetAnnouncementsError;
#[automock]
#[async_trait]
pub trait AnnouncementRepository: Send + Sync {
    async fn get_announcements(&self) -> Result<Vec<AnnouncementDTO>, GetAnnouncementsError>;
}