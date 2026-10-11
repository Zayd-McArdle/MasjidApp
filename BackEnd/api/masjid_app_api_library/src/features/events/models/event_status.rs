use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, PartialEq, Eq)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Confirmed,
    Cancelled,
}
impl Display for EventStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EventStatus::Confirmed => "confirmed".to_owned(),
            EventStatus::Cancelled => "cancelled".to_owned(),
        };
        write!(f, "{}", str)
    }
}
impl FromStr for EventStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "confirmed" => Ok(EventStatus::Confirmed),
            "cancelled" => Ok(EventStatus::Cancelled),
            _ => Err(()),
        }
    }
}
