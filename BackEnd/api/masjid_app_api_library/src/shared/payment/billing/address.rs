use crate::shared::payment::billing::postal_code::PostalCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Address {
    #[validate(length(min = 2))]
    pub line_1: String,
    #[validate(length(min = 2))]
    pub line_2: Option<String>,
    #[validate(length(min = 2))]
    pub city: String,
    #[validate(length(min = 2))]
    pub region: String,
    #[validate(length(min = 2))]
    pub country: Option<String>,
    #[validate(nested)]
    pub postal_code: PostalCode,
}
