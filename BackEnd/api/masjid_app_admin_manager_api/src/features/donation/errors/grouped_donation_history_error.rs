use sqlx::Error;

pub enum GroupedDonationHistoryError {
    UnableToGetGroupedDonationHistory,
}
impl From<Error> for GroupedDonationHistoryError {
    fn from(value: Error) -> Self {
        Self::UnableToGetGroupedDonationHistory
    }
}
