use serde::Deserialize;
use validator::{Validate, ValidationError};
#[derive(Deserialize, Validate, Clone)]
pub struct RegistrationRequest {
    #[validate(length(min = 2, message = "First name cannot be empty"))]
    #[serde(rename(deserialize = "fullName"))]
    pub full_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "validate_role"))]
    pub role: String,
    #[validate(length(min = 2, message = "Please enter a valid username"))]
    pub username: String,
    #[validate(length(
        min = 16,
        message = "Password length must be a minimum of 16 characters"
    ))]
    pub password: String,
}
#[inline]
fn validate_role(role: &str) -> Result<(), ValidationError> {
    if role == "Admin" || role == "Imam" {
        return Ok(());
    }
    Err(ValidationError::new("Invalid role"))
}
