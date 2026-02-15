use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
            DonationIntention::Lillah => "Lillah".to_owned(),
            DonationIntention::Sadaqah => "Sadaqah".to_owned(),
            DonationIntention::Zakat => "Zakat".to_owned(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for DonationIntention {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lillah" => Ok(Self::Lillah),
            "Sadaqah" => Ok(Self::Sadaqah),
            "Zakat" => Ok(Self::Zakat),
            _ => Err(()),
        }
    }
}
