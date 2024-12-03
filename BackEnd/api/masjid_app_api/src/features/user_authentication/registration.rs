use std::future::Future;
use axum::extract::State;
use axum::http::{StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use validator::{Validate, ValidationError};
use crate::features::user_authentication::login::LoginRequest;
use crate::shared::app_state::AppState;
use crate::shared::user_management::UserAccount;

#[derive(Deserialize, Validate)]
pub struct RegistrationRequest {
    #[validate(length(min = 2, message = "First name cannot be empty"))]
    pub first_name: String,
    #[validate(length(min = 2, message = "Last name cannot be empty"))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "validate_role"))]
    pub role: String,
    #[validate(length(min = 2, message = "Please enter a valid username"))]
    pub username: String,
    #[validate(length(min = 16, message = "Password length must be a minimum of 16 characters"))]
    pub password: String,
}
pub struct UserAccountDTO {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub username: String,
    pub password: String,
}
fn validate_role(role: &str) -> Result<(), ValidationError> {
    if role == "Admin" || role == "Imam" {
        return Ok(());
    }
    Err(ValidationError::new("Invalid role"))
}
pub enum RegistrationError {
    UserAlreadyRegistered,
    FailedToRegister
}
pub(crate) fn register_user(State(state): State<AppState>, Json(request): Json<RegistrationRequest>) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    match state.user_repository.register_user(UserAccountDTO {
        first_name: request.first_name,
        last_name: request.last_name,
        email: request.email,
        role: request.role,
        username: request.username,
        password: request.password,
    }) {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(RegistrationError::UserAlreadyRegistered) => StatusCode::CONFLICT.into_response(),
        Err(..) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}