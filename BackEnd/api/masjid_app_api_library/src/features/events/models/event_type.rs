use std::fmt::Display;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, sqlx::Type, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Talk,
    Social,
    Class,
}
impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EventType::Talk => "talk".to_owned(),
            EventType::Social => "social".to_owned(),
            EventType::Class => "class".to_owned(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for EventType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "talk" => Ok(EventType::Talk),
            "social" => Ok(EventType::Social),
            "class" => Ok(EventType::Class),
            _ => Err(()),
        }
    }
}
