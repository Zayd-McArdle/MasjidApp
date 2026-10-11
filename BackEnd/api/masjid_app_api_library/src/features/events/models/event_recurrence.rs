use std::fmt::Display;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
pub enum EventRecurrence {
    OneOff,
    Daily,
    Weekly,
    Fortnightly,
    Monthly,
}

impl Display for EventRecurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EventRecurrence::OneOff => "one-off".to_owned(),
            EventRecurrence::Daily => "daily".to_owned(),
            EventRecurrence::Weekly => "weekly".to_owned(),
            EventRecurrence::Fortnightly => "fortnightly".to_owned(),
            EventRecurrence::Monthly => "monthly".to_owned(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for EventRecurrence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one-off" => Ok(EventRecurrence::OneOff),
            "daily" => Ok(EventRecurrence::Daily),
            "weekly" => Ok(EventRecurrence::Weekly),
            "fortnight" => Ok(EventRecurrence::Fortnightly),
            "monthly" => Ok(EventRecurrence::Monthly),
            _ => Err(()),
        }
    }
}
