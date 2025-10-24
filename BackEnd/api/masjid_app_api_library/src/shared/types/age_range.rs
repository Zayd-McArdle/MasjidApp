use serde::{Deserialize, Serialize};
use std::fmt::Display;
use validator::{Validate, ValidationErrors};
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AgeRange {
    #[serde(rename(serialize = "minimumAge", deserialize = "minimumAge"))]
    pub minimum_age: u8,
    #[serde(rename(serialize = "maximumAge", deserialize = "maximumAge"))]
    pub maximum_age: u8,
}
impl Validate for AgeRange {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.minimum_age <= self.maximum_age || (self.minimum_age != 0 && self.maximum_age != 0)
        {
            return Ok(());
        }
        Err(ValidationErrors::new())
    }
}
impl Display for AgeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("{}-{}", self.minimum_age, self.maximum_age)
        )
    }
}
