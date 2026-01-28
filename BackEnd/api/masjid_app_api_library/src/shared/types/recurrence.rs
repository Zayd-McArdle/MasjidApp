use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
pub enum Recurrence {
    OneOff,
    Daily,
    Weekly,
    Fortnightly,
    Monthly,
}
impl Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let recurrence = match self {
            Recurrence::OneOff => "one-off".to_owned(),
            Recurrence::Daily => "daily".to_owned(),
            Recurrence::Weekly => "weekly".to_owned(),
            Recurrence::Fortnightly => "fortnightly".to_owned(),
            Recurrence::Monthly => "monthly".to_owned(),
        };
        write!(f, "{}", recurrence)
    }
}
impl FromStr for Recurrence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one-off" => Ok(Recurrence::OneOff),
            "daily" => Ok(Recurrence::Daily),
            "weekly" => Ok(Recurrence::Weekly),
            "fortnightly" => Ok(Recurrence::Fortnightly),
            "monthly" => Ok(Recurrence::Monthly),
            _ => Err(()),
        }
    }
}
