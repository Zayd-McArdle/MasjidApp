use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BillingAddress {
    #[serde(rename = "line1")]
    #[validate(length(min = 2))]
    pub line_1: String,

    #[serde(rename = "line2")]
    #[validate(length(min = 2))]
    pub line_2: Option<String>,

    #[validate(length(min = 2))]
    pub city: String,

    #[validate(length(min = 2))]
    pub region: String,

    #[validate(length(min = 2))]
    pub country: Option<String>,

    #[serde(rename = "postalCode")]
    #[validate(length(min = 5, max = 12))]
    pub postal_code: String,
}
