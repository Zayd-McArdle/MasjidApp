use serde::Deserialize;
use validator::Validate;
#[derive(Deserialize, Validate, Clone)]
pub struct ResetUserPasswordRequest {
    #[validate(length(min = 2))]
    pub username: String,
    #[validate(length(min = 16))]
    pub replacement_password: String,
}
