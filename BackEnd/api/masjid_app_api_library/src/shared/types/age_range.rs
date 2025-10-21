use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct AgeRange {
    #[serde(rename(serialize = "minimumAge", deserialize = "minimumAge"))]
    pub minimum_age:u8,
    #[serde(rename(serialize = "maximumAge", deserialize = "maximumAge"))]
    pub maximum_age:u8,
}