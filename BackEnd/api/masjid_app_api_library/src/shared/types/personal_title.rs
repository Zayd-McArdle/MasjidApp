use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum PersonalTitle {
    Mr,
    Mrs,
    Ms,
}

impl Display for PersonalTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = match self {
            PersonalTitle::Mr => "Mr",
            PersonalTitle::Mrs => "Mrs",
            PersonalTitle::Ms => "Ms",
        };
        write!(f, "{}", title)
    }
}
impl FromStr for PersonalTitle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "mr" => Ok(PersonalTitle::Mr),
            "mrs" => Ok(PersonalTitle::Mrs),
            "ms" => Ok(PersonalTitle::Ms),
            _ => Err(()),
        }
    }
}
