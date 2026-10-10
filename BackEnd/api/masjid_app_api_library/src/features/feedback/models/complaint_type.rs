use crate::features::feedback::models::{constants::FACILITIES, facility::Facility};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

const STAFF: &'static str = "Staff";
const MANAGERIAL: &'static str = "Managerial";
const IMAM: &'static str = "Imam";
const MANAGEMENT: &'static str = "Management";
const WORSHIPPER: &'static str = "Worshipper";
const NEIGHBOUR_OBSTRUCTION: &'static str = "Neighbour Obstruction";
#[derive(Deserialize, Serialize)]
pub enum ComplaintType {
    Staff,
    Managerial,
    Imam,
    Management,
    Worshipper,
    NeighbourObstruction,
    Facilities(Facility),
    Other(String),
}

impl Display for ComplaintType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let complaint_type_str = match self {
            ComplaintType::Staff => STAFF,
            ComplaintType::Managerial => MANAGERIAL,
            ComplaintType::Imam => IMAM,
            ComplaintType::Management => MANAGEMENT,
            ComplaintType::Worshipper => WORSHIPPER,
            ComplaintType::NeighbourObstruction => NEIGHBOUR_OBSTRUCTION,
            ComplaintType::Facilities(facility) => &facility.to_string(),
            ComplaintType::Other(other) => other,
        };
        write!(f, "{}", complaint_type_str)
    }
}

impl FromStr for ComplaintType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            STAFF => Ok(Self::Staff),
            MANAGERIAL => Ok(Self::Managerial),
            IMAM => Ok(Self::Imam),
            MANAGEMENT => Ok(Self::Management),
            WORSHIPPER => Ok(Self::Worshipper),
            NEIGHBOUR_OBSTRUCTION => Ok(Self::NeighbourObstruction),
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
