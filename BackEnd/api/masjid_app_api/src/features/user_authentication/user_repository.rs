use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{mysql, Error};
use crate::features::user_authentication::login::{LoginDTO, LoginError};
use super::login;
use bcrypt;
use sqlx::mysql::MySqlQueryResult;
use crate::features::user_authentication::login::LoginError::{InvalidCredentials, UnableToLogin};
use crate::features::user_authentication::registration::{RegistrationError, UserAccountDTO};
use crate::features::user_authentication::reset_password::ResetPasswordError;
use crate::shared::repository_manager::{ConnectionString, Repository};
#[async_trait]
pub trait UserRepository {
    async fn login(&self, username: String, password: String) -> Result<LoginDTO, LoginError>;
    async fn register_user(&self, new_user: UserAccountDTO) -> Result<(), RegistrationError>;
    async fn reset_user_password(&self, username: String, new_password: String) -> Result<(), &str>;
}
impl UserRepository for Repository {
    async fn login(&self, username: String, password: String) -> Result<(), LoginError> {
        let user_credentials: Result<LoginDTO, Error> = sqlx::query_as("CALL get_user_credentials(?)").bind(username).fetch_one(&self.db_connection).await;
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
        let query_result = sqlx::query("CALL register_user(?, ?, ?, ?, ?, ?);")
            .bind(new_user.first_name)
            .bind(new_user.last_name)
            .bind(new_user.role)
            .bind(new_user.email)
            .bind(new_user.username)
            .bind(new_user.password)
            .execute(&self.db_connection).await;
        match query_result {
            Ok(_) => Ok(()),
            Err(Error::Database(db_err)) if db_err.code().as_deref() == Some("1062") => Err(RegistrationError::UserAlreadyRegistered),
            Err(_) => Err(RegistrationError::FailedToRegister),
        }

    }
    async fn reset_user_password(&self, username: String, new_password: String) -> Result<(), ResetPasswordError> {
        let query_result = sqlx::query("CALL reset_user_password(?, ?);")
            .bind(username)
            .bind(new_password)
            .execute(&self.db_connection).await;
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

pub async fn new_user_repository() -> Arc<dyn UserRepository>
{
    Arc::new(Repository::new(ConnectionString::AuthenticationConnection).await)
}
