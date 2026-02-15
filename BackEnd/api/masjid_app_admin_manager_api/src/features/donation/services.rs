use crate::features::donation::errors::donation_history_service_errors::GetDonationTransactionHistoryError;
use crate::features::donation::errors::grouped_donation_history_error::GroupedDonationHistoryError;
use crate::features::donation::models::donation_dto::DonationHistoryDTO;
use crate::features::donation::models::donation_filter::DonationFilter;
use crate::features::donation::models::donation_grouping::DonationGrouping;
use crate::features::donation::repositories::DonationHistoryAdminRepository;
use async_trait::async_trait;
use masjid_app_api_library::get_from_repositories_common;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use mockall::automock;
use std::collections::HashMap;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait DonationHistoryService: Send + Sync {
    async fn get_donation_transaction_history(
        &self,
        donation_filter: DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionHistoryError>;
    async fn get_grouped_donation_transaction_history(
        &self,
        donation_filter: DonationFilter,
        donation_grouping: DonationGrouping,
    ) -> Result<HashMap<String, Vec<DonationHistoryDTO>>, GroupedDonationHistoryError>;
    async fn get_grouped_donation_transaction_history_count(
        &self,
        donation_filter: DonationFilter,
        donation_grouping: DonationGrouping,
    ) -> Result<HashMap<String, u32>, GroupedDonationHistoryError>;
}

#[async_trait]
impl DonationHistoryService for CommonServiceImpl<dyn DonationHistoryAdminRepository> {
    async fn get_donation_transaction_history(
        &self,
        donation_filter: DonationFilter,
    ) -> Result<Vec<DonationHistoryDTO>, GetDonationTransactionHistoryError> {
        get_from_repositories_common!(self, get_donation_transactions(&donation_filter))
    }

    async fn get_grouped_donation_transaction_history(
        &self,
        donation_filter: DonationFilter,
        donation_grouping: DonationGrouping,
    ) -> Result<HashMap<String, Vec<DonationHistoryDTO>>, GroupedDonationHistoryError> {
        todo!()
    }

    async fn get_grouped_donation_transaction_history_count(
        &self,
        donation_filter: DonationFilter,
        donation_grouping: DonationGrouping,
    ) -> Result<HashMap<String, u32>, GroupedDonationHistoryError> {
        get_from_repositories_common!(
            self,
            get_grouped_donation_transaction_history_count(&donation_filter, &donation_grouping)
        )
    }
}
pub async fn new_donation_history_service(
    repository: Arc<dyn DonationHistoryAdminRepository>,
    in_memory_repository: Arc<dyn DonationHistoryAdminRepository>,
) -> Arc<dyn DonationHistoryService> {
    Arc::new(CommonServiceImpl::<dyn DonationHistoryAdminRepository> {
        repository,
        in_memory_repository,
    })
}

#[cfg(test)]
mod tests {
    use crate::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;
    use crate::features::donation::errors::donation_history_service_errors::GetDonationTransactionHistoryError;
    use crate::features::donation::models::donation_dto::DonationHistoryDTO;
    use crate::features::donation::models::donation_filter::DonationFilter;
    use crate::features::donation::repositories::{
        DonationHistoryAdminRepository, MockDonationHistoryAdminRepository,
    };
    use crate::features::donation::services::DonationHistoryService;
    use masjid_app_api_library::features::donation::models::donation_details::DonationDetails;
    use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
    use masjid_app_api_library::shared::payment::billing_address::BillingAddress;
    use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_donation_history_service_get_donation_transaction_history() {
        struct TestCase {
            description: &'static str,
            expected_in_memory_repository_result:
                Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError>,
            expected_repository_result:
                Option<Result<Vec<DonationHistoryDTO>, GetDonationTransactionsError>>,
            expected_result: Result<Vec<DonationHistoryDTO>, GetDonationTransactionHistoryError>,
        }
        let donation_history = vec![DonationHistoryDTO {
            id: 0,
            donation_details: DonationDetails::default(),
            donor_address: BillingAddress::default(),
            transaction_status: TransactionStatus::Approved,
        }];
        let test_cases = [
            TestCase {
                description: "When record retrieval fails from both repositories, I should get an error",
                expected_in_memory_repository_result: Err(
                    GetDonationTransactionsError::UnableToFetchDonationTransactions,
                ),
                expected_repository_result: Some(Err(
                    GetDonationTransactionsError::UnableToFetchDonationTransactions,
                )),
                expected_result: Err(
                    GetDonationTransactionHistoryError::UnableToFetchRecordsFromRepository,
                ),
            },
            TestCase {
                description: "When there are empty records in both repositories, I should get an error",
                expected_in_memory_repository_result: Err(GetDonationTransactionsError::NotFound),
                expected_repository_result: Some(Err(GetDonationTransactionsError::NotFound)),
                expected_result: Err(GetDonationTransactionHistoryError::NoRecordsFound),
            },
            TestCase {
                description: "When an record retrieval fails in the in-memory repository but the main repository returns empty records, I should get an error",
                expected_in_memory_repository_result: Err(
                    GetDonationTransactionsError::UnableToFetchDonationTransactions,
                ),
                expected_repository_result: Some(Err(GetDonationTransactionsError::NotFound)),
                expected_result: Err(GetDonationTransactionHistoryError::NoRecordsFound),
            },
            TestCase {
                description: "When an error occurs in the in-memory repository but records are successfully retrieved in the main repository, I should get no error",
                expected_in_memory_repository_result: Err(
                    GetDonationTransactionsError::UnableToFetchDonationTransactions,
                ),
                expected_repository_result: Some(Ok(donation_history.clone())),
                expected_result: Ok(donation_history.clone()),
            },
            TestCase {
                description: "When records are successfully retrieved in the in-memory repository, I should get no error",
                expected_in_memory_repository_result: Ok(donation_history.clone()),
                expected_repository_result: None,
                expected_result: Ok(donation_history.clone()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_in_memory_repository = MockDonationHistoryAdminRepository::new();
            let mut mock_repository = MockDonationHistoryAdminRepository::new();

            mock_in_memory_repository
                .expect_get_donation_transactions()
                .return_once(|_| test_case.expected_in_memory_repository_result);

            if let Some(expected_repository_result) = test_case.expected_repository_result {
                mock_repository
                    .expect_get_donation_transactions()
                    .return_once(|_| expected_repository_result);
            }

            let donation_service = CommonServiceImpl::<dyn DonationHistoryAdminRepository> {
                repository: Arc::new(mock_repository),
                in_memory_repository: Arc::new(mock_in_memory_repository),
            };
            let _actual_result = donation_service
                .get_donation_transaction_history(DonationFilter::default())
                .await;
            assert!(matches!(test_case.expected_result, _actual_result));
        }
    }
}
