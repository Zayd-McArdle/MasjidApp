use crate::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;

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

impl From<sqlx::Error> for GroupedDonationHistoryError {
    fn from(value: sqlx::Error) -> Self {
        Self::UnableToGetGroupedDonationHistory
    }
}
