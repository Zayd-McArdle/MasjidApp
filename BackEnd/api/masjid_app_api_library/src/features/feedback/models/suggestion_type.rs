use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::features::feedback::models::constants::MANAGEMENT;
use crate::features::feedback::models::facility::Facility;
const PROPOSAL: &'static str = "Proposal";
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum SuggestionType {
    Proposal,
    Management,
    Facilities(Facility),
    Other(String),
}

impl Display for SuggestionType {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suggestion_type_str = match self {
            SuggestionType::Proposal => PROPOSAL,
            SuggestionType::Management => MANAGEMENT,
            SuggestionType::Facilities(facility) => &facility.to_string(),
            SuggestionType::Other(other) => other,
        };
        write!(f, "{}", suggestion_type_str)
    }
}

impl FromStr for SuggestionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PROPOSAL => Ok(SuggestionType::Proposal),
            MANAGEMENT => Ok(Self::Management),
            other => {
                Ok(Facility::from_str(other)
                    .map_or(Self::Other(other.to_owned()), Self::Facilities))
            }
        }
    }
}
