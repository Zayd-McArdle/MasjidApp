use masjid_app_api_library::shared::payment::errors::PaymentServiceError;

pub enum SendDonationServiceError {
    DonationHistoryPublicRepositoryError(InsertDonationTransactionError),
    PaymentServiceError(PaymentServiceError),
    DatabaseError,
}

pub enum InsertDonationTransactionError {
    UnableToInsertTransaction,
}
