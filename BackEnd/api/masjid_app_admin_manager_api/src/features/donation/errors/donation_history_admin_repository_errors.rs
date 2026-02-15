#[derive(Debug)]
pub enum GetDonationTransactionsError {
    NotFound,
    UnableToFetchDonationTransactions,
}
