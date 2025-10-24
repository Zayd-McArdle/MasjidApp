use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Validate, Eq)]
pub struct ContactDetails {
    #[validate(length(min = 3))]
    #[serde(rename(serialize = "fullName", deserialize = "fullName"))]
    pub full_name: String,
    #[serde(rename(serialize = "phoneNumber", deserialize = "phoneNumber"))]
    pub phone_number: String,
    #[validate(email)]
    pub email: Option<String>,
}
