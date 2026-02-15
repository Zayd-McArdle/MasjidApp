use crate::shared::traits::value_retriever::ValueRetriever;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;

const LILLAH: &'static str = "Lillah";
const SADAQAH: &'static str = "Sadaqah";
const ZAKAT: &'static str = "Zakat";

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum DonationIntention {
    Lillah,
    Sadaqah,
    Zakat,
}
impl Default for DonationIntention {
    fn default() -> Self {
        Self::Sadaqah
    }
}
impl Display for DonationIntention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DonationIntention::Lillah => LILLAH.to_owned(),
            DonationIntention::Sadaqah => SADAQAH.to_owned(),
            DonationIntention::Zakat => ZAKAT.to_owned(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for DonationIntention {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            LILLAH => Ok(Self::Lillah),
            SADAQAH => Ok(Self::Sadaqah),
            ZAKAT => Ok(Self::Zakat),
            _ => Err(()),
        }
    }
}

impl ValueRetriever for DonationIntention {
    fn get_values() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![Self::Lillah, Self::Sadaqah, Self::Zakat]
    }
}
