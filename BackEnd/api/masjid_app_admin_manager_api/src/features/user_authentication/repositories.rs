use crate::features::user_authentication::errors::{
    GetUserError, InsertNewUserError, UpdateUserPasswordError,
};
use crate::features::user_authentication::models::{LoginDTO, UserAccountDTO};
use async_trait::async_trait;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use masjid_app_api_library::shared::data_access::repository_management::repository_type::RepositoryType;
use mockall::automock;
use sqlx::{Error, Row};
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_credentials(
        &self,
        username: &str,
        password: &str,
    ) -> Result<String, GetUserError>;
    async fn insert_new_user(&self, new_user: UserAccountDTO) -> Result<(), InsertNewUserError>;
    async fn update_user_password(
        &self,
        username: &str,
        new_password: &str,
    ) -> Result<(), UpdateUserPasswordError>;
}
pub async fn new_user_repository() -> Arc<dyn UserRepository> {
    Arc::new(MySqlRepository::new(RepositoryType::Authentication).await)
}

#[async_trait]
impl UserRepository for MySqlRepository {
    async fn get_user_by_credentials(
        &self,
        username: &str,
        password: &str,
    ) -> Result<String, GetUserError> {
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
                    return GetUserError::NotFound;
                }
                tracing::error!(
                    username = username,
                    error = err.to_string(),
                    "an error occurred whilst registering new user",
                );
                GetUserError::DatabaseError
            })?;
        let hash_verified = bcrypt::verify(password, &user.password).map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                "unable to verify hash due to the following error"
            );
            GetUserError::UnableToVerifyPasswordHash
        })?;
        if hash_verified {
            tracing::info!(username = username, "logged in");
            return Ok(user.role);
        }
        tracing::debug!(
            username = username,
            "hashed password does not match hash in database"
        );
        Err(GetUserError::NotFound)
    }
    async fn insert_new_user(&self, new_user: UserAccountDTO) -> Result<(), InsertNewUserError> {
        let db_connection = self.db_connection.clone();
        let hashed_password = bcrypt::hash(new_user.password, 12).map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                "an error occurred when hashing the password"
            );
            InsertNewUserError::DatabaseError
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
                    return InsertNewUserError::UserExists;
                }
                tracing::error!(
                    username = new_user.username,
                    error = err.to_string(),
                    "an error occurred whilst registering user",
                );
                InsertNewUserError::DatabaseError
            })?;
        tracing::info!(username = new_user.username, "user successfully registered");
        Ok(())
    }
    async fn update_user_password(
        &self,
        username: &str,
        new_password: &str,
    ) -> Result<(), UpdateUserPasswordError> {
        let db_connection = self.db_connection.clone();
        let hashed_password = bcrypt::hash(new_password, 12).map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                "an error occurred when hashing the password"
            );
            UpdateUserPasswordError::DatabaseError
        })?;
        let query_result = sqlx::query("CALL reset_user_password(?, ?);")
            .bind(username)
            .bind(hashed_password)
            .execute(&*db_connection)
            .await
            .map_err(|_| UpdateUserPasswordError::DatabaseError)?;
        if query_result.rows_affected() == 0 {
            tracing::debug!(username = username, "user does not exist");
            return Err(UpdateUserPasswordError::UserDoesNotExist);
        }
        tracing::debug!(username = username, "successfully reset password");
        Ok(())
    }
}
