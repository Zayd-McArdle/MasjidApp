use serde::Deserialize;
use validator::Validate;
#[derive(Deserialize, Validate, Clone)]
pub struct LoginRequest {
    #[validate(length(min = 2))]
    pub username: String,
    #[validate(length(min = 2))]
    pub password: String,
}
