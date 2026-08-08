use crate::features::user_authentication::errors::get_user_error::GetUserError;
use crate::features::user_authentication::errors::insert_new_user_error::InsertNewUserError;
use crate::features::user_authentication::errors::update_user_password_error::UpdateUserPasswordError;
use crate::features::user_authentication::models::login_dto::LoginDTO;
use crate::features::user_authentication::models::user_account_dto::UserAccountDTO;
use async_trait::async_trait;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use masjid_app_api_library::shared::data_access::repository_management::repository_type::RepositoryType;
use mockall::automock;
use sqlx::Row;
use std::sync::Arc;

mod mysql_impl;
#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_credentials(
        &self,
        username: &str,
        password: &str,
    ) -> Result<LoginDTO, GetUserError>;
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
