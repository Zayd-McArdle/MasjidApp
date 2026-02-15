use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
pub enum RefundStatus {
    Requested,
    InProgress,
    Completed,
}

impl Display for RefundStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{:?}", self)
    }
}
