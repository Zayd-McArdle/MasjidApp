use masjid_app_api_library::shared::payment::errors::PaymentServiceError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SendDonationServiceError {
    DonationHistoryPublicRepositoryError(InsertDonationTransactionError),
    DonationHistoryPublicInMemoryRepositoryError(InsertDonationTransactionError),
    PaymentServiceError(PaymentServiceError),
    DatabaseError,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InsertDonationTransactionError {
    UnableToInsertTransaction,
}
