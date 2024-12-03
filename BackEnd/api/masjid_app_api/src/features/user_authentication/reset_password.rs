use std::os::macos::raw::stat;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use validator::Validate;
use crate::features::user_authentication::registration::RegistrationRequest;
use crate::shared::app_state::AppState;

#[derive(Validate)]
pub struct ResetUserPasswordRequest {
    #[validate(length(min = 2))]
    username: String,
    #[validate(length(min = 16))]
    replacement_password: String,
}
pub enum ResetPasswordError {
    UserDoesNotExist,
    FailedToResetUserPassword
}

pub async fn reset_user_password(State(state): State<AppState>, Json(request): Json<ResetUserPasswordRequest>) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    match state.user_repository.reset_user_password(request.username, request.replacement_password) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(ResetPasswordError::UserDoesNotExist) => StatusCode::NOT_FOUND.into_response(),
        Err(ResetPasswordError::FailedToResetUserPassword) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}