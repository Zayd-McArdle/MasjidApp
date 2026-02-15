use crate::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;

pub enum GetDonationTransactionHistoryError {
    NoRecordsFound,
    UnableToFetchRecordsFromRepository,
}

impl From<GetDonationTransactionsError> for GetDonationTransactionHistoryError {
    #[inline]
    fn from(value: GetDonationTransactionsError) -> Self {
        match value {
            GetDonationTransactionsError::NotFound => {
                GetDonationTransactionHistoryError::NoRecordsFound
            }
            GetDonationTransactionsError::UnableToFetchDonationTransactions => {
                GetDonationTransactionHistoryError::UnableToFetchRecordsFromRepository
            }
        }
    }
}
