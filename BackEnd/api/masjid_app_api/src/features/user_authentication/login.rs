use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use validator::Validate;
use crate::shared::app_state::AppState;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 2))]
    pub username: String,
    #[validate(length(min = 2))]
    pub password: String,
}


#[derive(sqlx::FromRow)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}

pub enum LoginError {
    InvalidCredentials,
    UnableToLogin
}

pub async fn login(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> Response {
    if let Err(_) = request.validate() {
        return (StatusCode::BAD_REQUEST, "The username or password cannot be empty").into_response();
    }
    match state.user_repository.login(request.username, request.password).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(LoginError::InvalidCredentials) => StatusCode::UNAUTHORIZED.into_response(),
        Err(LoginError::UnableToLogin) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }

}