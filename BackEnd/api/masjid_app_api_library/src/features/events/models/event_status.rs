use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, PartialEq, Eq)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Confirmed,
    Cancelled,
}
impl ToString for EventStatus {
    fn to_string(&self) -> String {
        match self {
            EventStatus::Confirmed => "confirmed".to_owned(),
            EventStatus::Cancelled => "cancelled".to_owned(),
        }
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
