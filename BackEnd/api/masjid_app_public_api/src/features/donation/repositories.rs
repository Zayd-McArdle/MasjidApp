use crate::features::donation::errors::InsertDonationTransactionError;
use async_trait::async_trait;
use masjid_app_api_library::features::donation::models::DonationDTO;

#[async_trait]
pub trait DonationHistoryPublicRepository: Send + Sync {
    async fn insert_donation_transaction(
        &self,
        donation: DonationDTO,
    ) -> Result<(), InsertDonationTransactionError>;
}
