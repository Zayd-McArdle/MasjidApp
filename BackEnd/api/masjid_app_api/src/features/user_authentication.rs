use std::sync::Arc;
use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use sqlx::{mysql, Error};
use bcrypt;
use serde::Deserialize;
use sqlx::mysql::MySqlQueryResult;
use validator::{Validate, ValidationError};
use crate::features::user_authentication::LoginError::{InvalidCredentials, UnableToLogin};
use crate::shared::app_state::InnerAppState;
use crate::shared::repository_manager::{ConnectionString, MainRepository, RepositoryMode};
#[async_trait]
pub trait UserRepository {
    async fn login(&self, username: String, password: String) -> Result<(), LoginError>;
    async fn register_user(&self, new_user: UserAccountDTO) -> Result<(), RegistrationError>;
    async fn reset_user_password(&self, username: String, new_password: String) -> Result<(), ResetPasswordError>;
}
#[async_trait]
impl UserRepository for MainRepository {
    async fn login(&self, username: String, password: String) -> Result<(), LoginError> {
        let db_connection = self.db_connection.clone();
        let user_credentials: Result<LoginDTO, Error> = sqlx::query_as("CALL get_user_credentials(?)")
        .bind(username)
        .fetch_one(&*db_connection).await;
        match user_credentials {
            Ok(user) => {
                let hash_verified = bcrypt::verify(password, &user.password).expect("unable to verify hash");
                if hash_verified {
                    return Ok(());
                }
                Err(InvalidCredentials)
            },
            Err(Error::RowNotFound) => Err(InvalidCredentials),
            Err(..) => Err(UnableToLogin)
        }

    }
    async fn register_user(&self, new_user: UserAccountDTO) -> Result<(), RegistrationError> {
        let db_connection = self.db_connection.clone();
        let query_result = sqlx::query("CALL register_user(?, ?, ?, ?, ?, ?);")
            .bind(new_user.first_name)
            .bind(new_user.last_name)
            .bind(new_user.role)
            .bind(new_user.email)
            .bind(new_user.username)
            .bind(new_user.password)
            .execute(&*db_connection).await;
        match query_result {
            Ok(_) => Ok(()),
            Err(Error::Database(db_err)) if db_err.code().as_deref() == Some("1062") => Err(RegistrationError::UserAlreadyRegistered),
            Err(_) => Err(RegistrationError::FailedToRegister),
        }

    }
    async fn reset_user_password(&self, username: String, new_password: String) -> Result<(), ResetPasswordError> {
        let db_connection = self.db_connection.clone();
        let query_result = sqlx::query("CALL reset_user_password(?, ?);")
            .bind(username)
            .bind(new_password)
            .execute(&*db_connection).await;
        match query_result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return Err(ResetPasswordError::UserDoesNotExist)
                }
                Ok(())
            },
            Err(_) => Err(ResetPasswordError::FailedToResetUserPassword),
        }
    }
}

pub async fn new_user_repository(mode: RepositoryMode) -> Arc<dyn UserRepository>
{
    Arc::new(MainRepository::new(ConnectionString::AuthenticationConnection).await)
}

//Login
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

pub async fn login(State(state): State<InnerAppState<Arc<dyn UserRepository>>>, Json(request): Json<LoginRequest>) -> Response {
    if let Err(_) = request.validate() {
        return (StatusCode::BAD_REQUEST, "The username or password cannot be empty").into_response();
    }
    match state.repositories[0].login(request.username, request.password).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(InvalidCredentials) => StatusCode::UNAUTHORIZED.into_response(),
        Err(UnableToLogin) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }

}

//Registration
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

#[derive(sqlx::FromRow)]
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
pub(crate) async fn register_user(State(state): State<InnerAppState<Arc<dyn UserRepository>>>, Json(request): Json<RegistrationRequest>) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    match state.repositories[0].register_user(UserAccountDTO {
        first_name: request.first_name,
        last_name: request.last_name,
        email: request.email,
        role: request.role,
        username: request.username,
        password: request.password,
    }).await {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(RegistrationError::UserAlreadyRegistered) => StatusCode::CONFLICT.into_response(),
        Err(..) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

//Password Reset
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

pub async fn reset_user_password(State(state): State<InnerAppState<Arc<dyn UserRepository>>>, Json(request): Json<ResetUserPasswordRequest>) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    match state.repositories[0].reset_user_password(request.username, request.replacement_password).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(ResetPasswordError::UserDoesNotExist) => StatusCode::NOT_FOUND.into_response(),
        Err(ResetPasswordError::FailedToResetUserPassword) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}