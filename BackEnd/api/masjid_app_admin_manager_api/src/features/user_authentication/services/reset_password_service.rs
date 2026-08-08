use crate::features::user_authentication::repositories::UserRepository;
use crate::features::user_authentication::services::authentication_service_impl::AuthenticationServiceImpl;
use crate::features::user_authentication::services::errors::reset_password_error::ResetPasswordError;
use crate::new_authentication_service;
use async_trait::async_trait;
use masjid_app_api_library::shared::services::hashing::r#trait::HashingService;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait ResetPasswordService: Send + Sync {
    async fn reset_password(
        &self,
        username: &str,
        new_password: &str,
    ) -> Result<(), ResetPasswordError>;
}
new_authentication_service!(new_reset_password_service, ResetPasswordService);

#[async_trait]
impl ResetPasswordService for AuthenticationServiceImpl {
    async fn reset_password(
        &self,
        username: &str,
        new_password: &str,
    ) -> Result<(), ResetPasswordError> {
        let hashed_password = self.hashing_service.hash(new_password.as_bytes())?;
        self.user_repository
            .update_user_password(username, &hashed_password)
            .await
            .map_err(ResetPasswordError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::user_authentication::errors::update_user_password_error::UpdateUserPasswordError;
    use crate::features::user_authentication::repositories::MockUserRepository;
    use masjid_app_api_library::shared::services::hashing::providers::HashingProvider;
    use masjid_app_api_library::shared::services::hashing::r#trait::new_hashing_service;

    #[tokio::test]
    async fn test_reset_password_service_reset_password() {
        struct TestCase {
            description: &'static str,
            mock_repository_result: Result<(), UpdateUserPasswordError>,
            expected_result: Result<(), ResetPasswordError>,
        }
        let test_cases = [
            TestCase {
                description: "When the repository fails to update the password, I should get an error",
                mock_repository_result: Err(UpdateUserPasswordError::DatabaseError),
                expected_result: Err(ResetPasswordError::UnableToResetPassword),
            },
            TestCase {
                description: "When resetting a password for a user that does not exist, I should get an error",
                mock_repository_result: Err(UpdateUserPasswordError::UserDoesNotExist),
                expected_result: Err(ResetPasswordError::UserDoesNotExist),
            },
            TestCase {
                description: "When the repository successfully updates the user password, I should get no error",
                mock_repository_result: Ok(()),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_repository = MockUserRepository::new();
            mock_repository
                .expect_update_user_password()
                .return_once(|_, _| test_case.mock_repository_result);
            let reset_password_service = new_reset_password_service(
                new_hashing_service(HashingProvider::Bcrypt),
                Arc::new(mock_repository),
            );
            let actual_result = reset_password_service.reset_password("", "");
            assert!(matches!(test_case.expected_result, actual_result));
        }
    }
}
