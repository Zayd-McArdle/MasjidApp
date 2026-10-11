use crate::features::feedback::models::{constants::FACILITIES, facility::Facility};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

const NEIGHBOUR_DISRUPTION: &'static str = "Neighbour Disruption";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ConcernType {
    NeighbourDisruption,
    Facilities(Facility),
    Other(String),
}

impl Display for ConcernType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let complaint_type_str = match self {
            Self::NeighbourDisruption => NEIGHBOUR_DISRUPTION,
            Self::Facilities(facility) => &facility.to_string(),
            Self::Other(other) => other,
        };
        write!(f, "{}", complaint_type_str)
    }
}

impl FromStr for ConcernType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            NEIGHBOUR_DISRUPTION => Ok(Self::NeighbourDisruption),
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
