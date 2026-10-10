use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::features::feedback::models::constants::{FACILITIES, MANAGEMENT};
use crate::features::feedback::models::facility::Facility;

#[derive(Deserialize, Serialize)]
pub enum SuggestionType {
    Management,
    Facilities(Facility),
    Other(String),
}

impl Display for SuggestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suggestion_type_str = match self {
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
            MANAGEMENT => Ok(Self::Management),
            other => {
                if other.starts_with(FACILITIES) {
                    return Ok(Self::Facilities(Facility::from_str(
                        &other.replace(FACILITIES, ""),
                    )?));
                }
                Ok(Self::Other(other.to_owned()))
            }
        }
    }
}
