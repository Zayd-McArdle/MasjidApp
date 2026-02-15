use crate::features::donation::errors::InsertDonationTransactionError;
use async_trait::async_trait;
use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::repository_manager::{
    InMemoryRepository, MySqlRepository, RepositoryMode, RepositoryType,
};
use mockall::automock;
use std::sync::Arc;
use validator::ValidateRange;

#[automock]
#[async_trait]
pub trait DonationHistoryPublicRepository: Send + Sync {
    async fn insert_donation_transaction(
        &self,
        donation: &DonationHistory,
    ) -> Result<(), InsertDonationTransactionError>;
}
#[async_trait]
impl DonationHistoryPublicRepository for InMemoryRepository {
    async fn insert_donation_transaction(
        &self,
        donation: &DonationHistory,
    ) -> Result<(), InsertDonationTransactionError> {
        todo!()
    }
}

#[async_trait]
impl DonationHistoryPublicRepository for MySqlRepository {
    async fn insert_donation_transaction(
        &self,
        donation: &DonationHistory,
    ) -> Result<(), InsertDonationTransactionError> {
        const STORED_PROCEDURE: &'static str =
            "CALL insert_donation_transaction(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let db_connection = self.db_connection.clone();
        let query_result = sqlx::query(STORED_PROCEDURE)
            .bind(&donation.cause)
            .bind(&donation.donation_intention)
            .bind(&donation.donor_full_name)
            .bind(&donation.donor_title)
            .bind(&donation.phone_number)
            .bind(&donation.email)
            .bind(&donation.address_line_1)
            .bind(&donation.address_line_2)
            .bind(&donation.address_city)
            .bind(&donation.address_region)
            .bind(&donation.address_country)
            .bind(&donation.address_postal)
            .bind(&donation.amount)
            .bind(&donation.is_gift_aid)
            .bind(&donation.donation_frequency)
            .bind(&donation.transaction_status)
            .execute(&*db_connection)
            .await
            .map_err(|err| {
                tracing::error!(
                    error = err.to_string(),
                    "unable to insert donation history into database"
                );
                InsertDonationTransactionError::UnableToInsertTransaction
            })?;

        if query_result.rows_affected() == 0 {
            return Err(InsertDonationTransactionError::UnableToInsertTransaction);
        }
        Ok(())
    }
}

pub async fn new_donation_history_public_repository(
    repository_mode: RepositoryMode,
) -> Arc<dyn DonationHistoryPublicRepository> {
    new_repository!(repository_mode, RepositoryType::Donation)
}
