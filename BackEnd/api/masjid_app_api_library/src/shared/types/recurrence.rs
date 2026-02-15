use crate::shared::traits::value_retriever::ValueRetriever;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
const ONE_OFF: &'static str = "one-off";
const DAILY: &'static str = "daily";
const WEEKLY: &'static str = "weekly";
const FORTNIGHTLY: &'static str = "fortnightly";
const MONTHLY: &'static str = "monthly";
const ANNUALLY: &'static str = "annually";

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
pub enum Recurrence {
    OneOff,
    Daily,
    Weekly,
    Fortnightly,
    Monthly,
    Annually,
}
impl Default for Recurrence {
    fn default() -> Self {
        Self::OneOff
    }
}
impl Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let recurrence = match self {
            Recurrence::OneOff => ONE_OFF.to_owned(),
            Recurrence::Daily => DAILY.to_owned(),
            Recurrence::Weekly => WEEKLY.to_owned(),
            Recurrence::Fortnightly => FORTNIGHTLY.to_owned(),
            Recurrence::Monthly => MONTHLY.to_owned(),
            Recurrence::Annually => ANNUALLY.to_owned(),
        };
        write!(f, "{}", recurrence)
    }
}
impl FromStr for Recurrence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ONE_OFF => Ok(Recurrence::OneOff),
            DAILY => Ok(Recurrence::Daily),
            WEEKLY => Ok(Recurrence::Weekly),
            FORTNIGHTLY => Ok(Recurrence::Fortnightly),
            MONTHLY => Ok(Recurrence::Monthly),
            ANNUALLY => Ok(Recurrence::Annually),
            _ => Err(()),
        }
    }
}

impl ValueRetriever for Recurrence {
    fn get_values() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Recurrence::OneOff,
            Recurrence::Daily,
            Recurrence::Weekly,
            Recurrence::Fortnightly,
            Recurrence::Monthly,
            Recurrence::Annually,
        ]
    }
}
