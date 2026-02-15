use std::fmt::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TransactionDeclinedReason {
    CardExpired,
    InsufficientFunds,
    CardBlocked,
    CardFrozen,
    SuspectedFraud,
}

impl Display for TransactionDeclinedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decline_string = match self {
            TransactionDeclinedReason::CardExpired => "declined because card expired",
            TransactionDeclinedReason::InsufficientFunds => "declined because insufficient funds",
            TransactionDeclinedReason::CardBlocked => "declined because card blocked",
            TransactionDeclinedReason::CardFrozen => "declined because card frozen",
            TransactionDeclinedReason::SuspectedFraud => "declined because suspected fraud",
        };
        write!(f, "{:?}", decline_string)
    }
}
