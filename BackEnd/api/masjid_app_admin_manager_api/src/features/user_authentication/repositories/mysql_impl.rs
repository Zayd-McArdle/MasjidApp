use crate::features::user_authentication::errors::get_user_error::GetUserError;
use crate::features::user_authentication::errors::insert_new_user_error::InsertNewUserError;
use crate::features::user_authentication::errors::update_user_password_error::UpdateUserPasswordError;
use crate::features::user_authentication::models::login_dto::LoginDTO;
use crate::features::user_authentication::models::user_account_dto::UserAccountDTO;
use crate::features::user_authentication::repositories::UserRepository;
use async_trait::async_trait;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use sqlx::{Error, Row};

#[async_trait]
impl UserRepository for MySqlRepository {
    async fn get_user_by_credentials(
        &self,
        username: &str,
        password: &str,
    ) -> Result<LoginDTO, GetUserError> {
        let db_connection = self.db_connection.clone();
        sqlx::query("CALL get_user_credentials(?)")
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
                    return GetUserError::NotFound;
                }
                tracing::error!(
                    username = username,
                    error = err.to_string(),
                    "an error occurred whilst retrieving user",
                );
                GetUserError::DatabaseError
            })
    }
    async fn insert_new_user(&self, new_user: UserAccountDTO) -> Result<(), InsertNewUserError> {
        let db_connection = self.db_connection.clone();
        sqlx::query("CALL register_user(?, ?, ?, ?, ?);")
            .bind(&new_user.full_name)
            .bind(&new_user.role)
            .bind(&new_user.email)
            .bind(&new_user.username)
            .bind(&new_user.password)
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
        let query_result = sqlx::query("CALL reset_user_password(?, ?);")
            .bind(username)
            .bind(new_password)
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
