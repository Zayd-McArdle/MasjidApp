use crate::features::user_authentication::errors::{
    LoginError, RegistrationError, ResetPasswordError,
};
use crate::features::user_authentication::models::{LoginDTO, UserAccountDTO};
use async_trait::async_trait;
use masjid_app_api_library::shared::data_access::repository_manager::{
    MySqlRepository, RepositoryType,
};
use mockall::automock;
use sqlx::{Error, Row};
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn login(&self, username: &str, password: &str) -> Result<String, LoginError>;
    async fn register_user(&self, new_user: UserAccountDTO) -> Result<(), RegistrationError>;
    async fn reset_user_password(
        &self,
        username: &str,
        new_password: &str,
    ) -> Result<(), ResetPasswordError>;
}
pub async fn new_user_repository() -> Arc<dyn UserRepository> {
    Arc::new(MySqlRepository::new(RepositoryType::Authentication).await)
}

#[async_trait]
impl UserRepository for MySqlRepository {
    async fn login(&self, username: &str, password: &str) -> Result<String, LoginError> {
        let db_connection = self.db_connection.clone();
        let user = sqlx::query("CALL get_user_credentials(?)")
            .bind(username)
            .map(|row: sqlx::mysql::MySqlRow| LoginDTO {
                username: row.get(0),
                password: row.get(1),
                role: row.get(2),
            })
            .fetch_one(&*db_connection)
            .await
            .map_err(|err| {
                if matches!(err, Error::RowNotFound) {
                    tracing::debug!(
                        username = username,
                        error = err.to_string(),
                        "user entered the wrong credentials"
                    );
                    return LoginError::InvalidCredentials;
                }
                tracing::error!(
                    username = username,
                    error = err.to_string(),
                    "an error occurred whilst registering new user",
                );
                LoginError::UnableToLogin
            })?;
        let hash_verified = bcrypt::verify(password, &user.password).map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                "unable to verify hash due to the following error"
            );
            LoginError::UnableToLogin
        })?;
        if hash_verified {
            tracing::info!(username = username, "logged in");
            return Ok(user.role);
        }
        tracing::debug!(
            username = username,
            "hashed password does not match hash in database"
        );
        Err(LoginError::InvalidCredentials)
    }
    async fn register_user(&self, new_user: UserAccountDTO) -> Result<(), RegistrationError> {
        let db_connection = self.db_connection.clone();
        let hashed_password = bcrypt::hash(new_user.password, 12).map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                "an error occurred when hashing the password"
            );
            RegistrationError::FailedToRegister
        })?;
        sqlx::query("CALL register_user(?, ?, ?, ?, ?);")
            .bind(&new_user.full_name)
            .bind(&new_user.role)
            .bind(&new_user.email)
            .bind(&new_user.username)
            .bind(hashed_password)
            .execute(&*db_connection)
            .await
            .map_err(|err| {
                if let Error::Database(ref db_err) = err
                    && db_err.is_unique_violation()
                {
                    tracing::debug!(username = new_user.username, "user already exists");
                    return RegistrationError::UserAlreadyRegistered;
                }
                tracing::error!(
                    username = new_user.username,
                    error = err.to_string(),
                    "an error occurred whilst registering user",
                );
                RegistrationError::FailedToRegister
            })?;
        tracing::info!(username = new_user.username, "user successfully registered");
        Ok(())
    }
    async fn reset_user_password(
        &self,
        username: &str,
        new_password: &str,
    ) -> Result<(), ResetPasswordError> {
        let db_connection = self.db_connection.clone();
        let hashed_password = bcrypt::hash(new_password, 12).map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                "an error occurred when hashing the password"
            );
            ResetPasswordError::FailedToResetUserPassword
        })?;
        let query_result = sqlx::query("CALL reset_user_password(?, ?);")
            .bind(username)
            .bind(hashed_password)
            .execute(&*db_connection)
            .await
            .map_err(|_| ResetPasswordError::FailedToResetUserPassword)?;
        if query_result.rows_affected() == 0 {
            tracing::debug!(username = username, "user does not exist");
            return Err(ResetPasswordError::UserDoesNotExist);
        }
        tracing::debug!(username = username, "successfully reset password");
        Ok(())
    }
}
