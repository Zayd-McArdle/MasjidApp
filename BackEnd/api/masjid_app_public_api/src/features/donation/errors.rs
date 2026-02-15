use masjid_app_api_library::shared::services::email::errors::SendEmailError;
use masjid_app_api_library::shared::services::payment::errors::PaymentServiceError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SendDonationServiceError {
    DonationHistoryPublicRepositoryError(InsertDonationTransactionError),
    DonationHistoryPublicInMemoryRepositoryError(InsertDonationTransactionError),
    PaymentServiceError(PaymentServiceError),
    EmailServiceFailure(SendEmailError),
}

pub enum RefundFailedDonationError {
    PaymentServiceError(PaymentServiceError),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InsertDonationTransactionError {
    UnableToInsertTransaction,
}
