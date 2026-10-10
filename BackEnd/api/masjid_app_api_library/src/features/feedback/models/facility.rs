use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
const TOILETS: &'static str = "Toilets";
const SINKS: &'static str = "Sinks";
const WUDHU_AREA: &'static str = "Wudhu Area";
const SHOWER: &'static str = "Shower";
const PRAYER_HALL: &'static str = "Prayer Hall";
const KITCHEN: &'static str = "Kitchen";

#[derive(Deserialize, Serialize)]
pub enum Facility {
    Toilets,
    Sinks,
    WudhuArea,
    Shower,
    PrayerHall,
    Kitchen,
    Other(String),
}

impl Display for Facility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let facility_str = match self {
            Facility::Toilets => TOILETS,
            Facility::Sinks => SINKS,
            Facility::WudhuArea => WUDHU_AREA,
            Facility::Shower => SHOWER,
            Facility::PrayerHall => PRAYER_HALL,
            Facility::Kitchen => KITCHEN,
            Facility::Other(other) => other,
        };
        write!(f, "{}", facility_str)
    }
}

impl FromStr for Facility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TOILETS => Ok(Facility::Toilets),
            SINKS => Ok(Facility::Sinks),
            WUDHU_AREA => Ok(Facility::WudhuArea),
            SHOWER => Ok(Facility::Shower),
            PRAYER_HALL => Ok(Facility::PrayerHall),
            KITCHEN => Ok(Facility::Kitchen),
            other => Ok(Facility::Other(other.to_owned())),
        }
    }
}