pub enum DonationHistoryDTOMappingError {
    FailedToMapDonationFrequency(String),
    FailedToMapTransactionStatus(String),
    FailedToMapDonationIntention(String),
}
