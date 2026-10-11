use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
pub(super) const TOILETS: &'static str = "Toilets";
pub(super) const SINKS: &'static str = "Sinks";
pub(super) const WUDHU_AREA: &'static str = "Wudhu Area";
pub(super) const SHOWER: &'static str = "Shower";
pub(super) const PRAYER_HALL: &'static str = "Prayer Hall";
pub(super) const KITCHEN: &'static str = "Kitchen";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Facility {
    Toilets,
    Sinks,
    WudhuArea,
    Shower,
    PrayerHall,
    Kitchen,
}

impl Display for Facility {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let facility_str = match self {
            Facility::Toilets => TOILETS,
            Facility::Sinks => SINKS,
            Facility::WudhuArea => WUDHU_AREA,
            Facility::Shower => SHOWER,
            Facility::PrayerHall => PRAYER_HALL,
            Facility::Kitchen => KITCHEN,
        };
        write!(f, "{}", facility_str)
    }
}

impl FromStr for Facility {
    type Err = ();
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TOILETS => Ok(Facility::Toilets),
            SINKS => Ok(Facility::Sinks),
            WUDHU_AREA => Ok(Facility::WudhuArea),
            SHOWER => Ok(Facility::Shower),
            PRAYER_HALL => Ok(Facility::PrayerHall),
            KITCHEN => Ok(Facility::Kitchen),
            _ => Err(()),
        }
    }
}
