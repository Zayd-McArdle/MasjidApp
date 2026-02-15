use crate::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;
use crate::features::donation::errors::grouped_donation_history_error::GroupedDonationHistoryError;
use crate::features::donation::models::donation_dto::DonationHistoryDTO;
use crate::features::donation::models::donation_filter::DonationFilter;
use crate::features::donation::models::donation_filter_with_grouping::DonationFilterWithGrouping;
use crate::features::donation::repositories::donation_history_admin_repository::DonationHistoryAdminRepository;
use async_trait::async_trait;
use masjid_app_api_library::shared::data_access::repository_manager::InMemoryRepository;
use std::collections::HashMap;

#[async_trait]
impl DonationHistoryAdminRepository for InMemoryRepository {
    async fn get_donation_transactions(
        &self,
        filters: &DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError> {
        todo!()
    }

    async fn get_grouped_donation_transaction_history(
        &self,
        donation_filter: &DonationFilterWithGrouping,
    ) -> Result<HashMap<String, Vec<DonationHistoryDTO>>, GroupedDonationHistoryError> {
        todo!()
    }

    async fn get_grouped_donation_transaction_history_count(
        &self,
        donation_filter: &DonationFilterWithGrouping,
    ) -> Result<HashMap<String, u32>, GroupedDonationHistoryError> {
        todo!()
    }
}
