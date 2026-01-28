use crate::features::donation::errors::SendDonationServiceError;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait DonationPublicService {
    async fn send_donation(&self, request: String) -> Result<(), SendDonationServiceError>;
}
