use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct ContactDetails {
    #[serde(rename(serialize = "fullName", deserialize = "fullName"))]
    pub full_name: String,
    #[serde(rename(serialize = "phoneNumber", deserialize = "phoneNumber"))]
    pub phone_number: String,
    pub email: Option<String>,

}