use crate::shared::payment::transaction_declined_reason::TransactionDeclinedReason;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TransactionStatus {
    Approved,
    Declined(TransactionDeclinedReason),
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_string = match self {
            TransactionStatus::Approved => "approved".to_owned(),
            TransactionStatus::Declined(declined_reason) => declined_reason.to_string(),
        };
        write!(f, "{:?}", status_string)
    }
}
