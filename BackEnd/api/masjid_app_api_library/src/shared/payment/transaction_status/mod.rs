mod constants;
pub mod transaction_declined_reason;
use crate::shared::payment::transaction_status::constants::APPROVED;
use crate::shared::payment::transaction_status::transaction_declined_reason::TransactionDeclinedReason;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
pub enum TransactionStatus {
    Approved,
    Declined(TransactionDeclinedReason),
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_string = match self {
            TransactionStatus::Approved => APPROVED.to_owned(),
            TransactionStatus::Declined(declined_reason) => declined_reason.to_string(),
        };
        write!(f, "{}", status_string)
    }
}

impl FromStr for TransactionStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            APPROVED => Ok(TransactionStatus::Approved),
            _ => TransactionDeclinedReason::from_str(s).map(TransactionStatus::Declined),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::payment::transaction_status::constants::{
        CARD_BLOCKED, CARD_EXPIRED, CARD_FROZEN, INSUFFICIENT_FUNDS, SUSPECTED_FRAUD,
    };
    use crate::shared::payment::transaction_status::TransactionStatus;
    use std::str::FromStr;

    #[test]
    fn transaction_status_from_str() {
        struct TestCase {
            transaction_status_str: &'static str,
            expected_status: Result<TransactionStatus, ()>,
        }

        let test_cases = [
            TestCase {
                transaction_status_str: APPROVED,
                expected_status: Ok(TransactionStatus::Approved),
            },
            TestCase {
                transaction_status_str: CARD_EXPIRED,
                expected_status: Ok(TransactionStatus::Declined(
                    TransactionDeclinedReason::CardExpired,
                )),
            },
            TestCase {
                transaction_status_str: CARD_BLOCKED,
                expected_status: Ok(TransactionStatus::Declined(
                    TransactionDeclinedReason::CardBlocked,
                )),
            },
            TestCase {
                transaction_status_str: CARD_FROZEN,
                expected_status: Ok(TransactionStatus::Declined(
                    TransactionDeclinedReason::CardFrozen,
                )),
            },
            TestCase {
                transaction_status_str: INSUFFICIENT_FUNDS,
                expected_status: Ok(TransactionStatus::Declined(
                    TransactionDeclinedReason::InsufficientFunds,
                )),
            },
            TestCase {
                transaction_status_str: SUSPECTED_FRAUD,
                expected_status: Ok(TransactionStatus::Declined(
                    TransactionDeclinedReason::SuspectedFraud,
                )),
            },
            TestCase {
                transaction_status_str: "",
                expected_status: Err(()),
            },
        ];
        for test_case in test_cases {
            let _actual_status = TransactionStatus::from_str(test_case.transaction_status_str);
            assert!(matches!(test_case.expected_status, _actual_status));
        }
    }
}
