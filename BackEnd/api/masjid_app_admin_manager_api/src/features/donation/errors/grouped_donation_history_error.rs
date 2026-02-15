use crate::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;

#[derive(Debug)]
pub enum GroupedDonationHistoryError {
    UnableToGetGroupedDonationHistory,
}
impl From<GetDonationTransactionsError> for GroupedDonationHistoryError {
    fn from(value: GetDonationTransactionsError) -> Self {
        match value {
            GetDonationTransactionsError::UnableToFetchDonationTransactions => {
                Self::UnableToGetGroupedDonationHistory
            }
        }
    }
}
