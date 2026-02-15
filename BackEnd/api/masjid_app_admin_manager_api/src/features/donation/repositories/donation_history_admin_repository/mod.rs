pub(super) mod mysql_impl;
pub(super) mod redis_impl;
pub(super) mod util;
use crate::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;
use crate::features::donation::errors::grouped_donation_history_error::GroupedDonationHistoryError;
use crate::features::donation::models::donation_dto::DonationHistoryDTO;
use crate::features::donation::models::donation_filter::DonationFilter;
use crate::features::donation::models::donation_filter_with_grouping::DonationFilterWithGrouping;
use async_trait::async_trait;
use masjid_app_api_library::new_repository;
use masjid_app_api_library::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use masjid_app_api_library::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use masjid_app_api_library::shared::data_access::repository_management::in_memory_repository::InMemoryRepository;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use masjid_app_api_library::shared::data_access::repository_management::repository_mode::RepositoryMode;
use masjid_app_api_library::shared::data_access::repository_management::repository_type::RepositoryType;
use mockall::automock;
use std::collections::HashMap;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait DonationHistoryAdminRepository: Send + Sync {
    async fn get_donation_transactions(
        &self,
        donation_filter: &DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError>;
    async fn get_grouped_donation_transaction_history(
        &self,
        donation_filter: &DonationFilterWithGrouping,
    ) -> Result<HashMap<String, Vec<DonationHistoryDTO>>, GroupedDonationHistoryError>;
    async fn get_grouped_donation_transaction_history_count(
        &self,
        donation_filter: &DonationFilterWithGrouping,
    ) -> Result<HashMap<String, i64>, GroupedDonationHistoryError>;
}

pub async fn new_donation_history_admin_repository(
    repository_mode: RepositoryMode,
) -> Arc<dyn DonationHistoryAdminRepository> {
    new_repository!(repository_mode, RepositoryType::Donation)
}
