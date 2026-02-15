use crate::shared::payment::transaction_status::constants::*;
use crate::shared::traits::value_retriever::ValueRetriever;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
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
            TransactionDeclinedReason::CardExpired => CARD_EXPIRED,
            TransactionDeclinedReason::InsufficientFunds => INSUFFICIENT_FUNDS,
            TransactionDeclinedReason::CardBlocked => CARD_BLOCKED,
            TransactionDeclinedReason::CardFrozen => CARD_FROZEN,
            TransactionDeclinedReason::SuspectedFraud => SUSPECTED_FRAUD,
        };
        write!(f, "{}", decline_string)
    }
}

impl FromStr for TransactionDeclinedReason {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decline_reason = match s {
            CARD_EXPIRED => TransactionDeclinedReason::CardExpired,
            INSUFFICIENT_FUNDS => TransactionDeclinedReason::InsufficientFunds,
            CARD_BLOCKED => TransactionDeclinedReason::CardBlocked,
            CARD_FROZEN => TransactionDeclinedReason::CardFrozen,
            SUSPECTED_FRAUD => TransactionDeclinedReason::SuspectedFraud,
            _ => return Err(()),
        };
        Ok(decline_reason)
    }
}

impl ValueRetriever for TransactionDeclinedReason {
    fn get_values() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::CardExpired,
            Self::InsufficientFunds,
            Self::CardBlocked,
            Self::CardFrozen,
            Self::SuspectedFraud,
        ]
    }
}
