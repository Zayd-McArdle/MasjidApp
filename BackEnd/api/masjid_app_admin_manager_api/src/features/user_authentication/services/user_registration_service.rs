use crate::features::user_authentication::models::user_account_dto::UserAccountDTO;
use crate::features::user_authentication::repositories::UserRepository;
use crate::features::user_authentication::services::authentication_service_impl::AuthenticationServiceImpl;
use crate::features::user_authentication::services::errors::user_registration_error::UserRegistrationError;
use crate::new_authentication_service;
use async_trait::async_trait;
use masjid_app_api_library::shared::services::hashing::r#trait::HashingService;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait UserRegistrationService: Send + Sync {
    async fn register_user(&self, new_user: UserAccountDTO) -> Result<(), UserRegistrationError>;
}

new_authentication_service!(new_user_registration_service, UserRegistrationService);

#[async_trait]
impl UserRegistrationService for AuthenticationServiceImpl {
    async fn register_user(
        &self,
        mut new_user: UserAccountDTO,
    ) -> Result<(), UserRegistrationError> {
        new_user.password = self.hashing_service.hash(&new_user.password.as_bytes())?;
        self.user_repository
            .insert_new_user(new_user)
            .await
            .map_err(UserRegistrationError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::user_authentication::errors::insert_new_user_error::InsertNewUserError;
    use crate::features::user_authentication::repositories::MockUserRepository;
    use masjid_app_api_library::shared::services::hashing::providers::HashingProvider;
    use masjid_app_api_library::shared::services::hashing::r#trait::new_hashing_service;

    #[tokio::test]
    async fn test_user_registration_service_register_user() {
        struct TestCase {
            description: &'static str,
            new_user: UserAccountDTO,
            mock_repository_result: Result<(), InsertNewUserError>,
            expected_result: Result<(), UserRegistrationError>,
        }
        let test_cases = [
            TestCase {
                description: "When registration fails, I should receive a database error",
                new_user: UserAccountDTO {
                    full_name: "".to_owned(),
                    email: "".to_owned(),
                    role: "".to_owned(),
                    username: "".to_owned(),
                    password: "".to_owned(),
                },
                mock_repository_result: Err(InsertNewUserError::DatabaseError),
                expected_result: Err(UserRegistrationError::UnableToRegisterToRepository),
            },
            TestCase {
                description: "When user already exists, I should a user exists error",

                new_user: UserAccountDTO {
                    full_name: "".to_owned(),
                    email: "".to_owned(),
                    role: "".to_owned(),
                    username: "".to_owned(),
                    password: "".to_owned(),
                },
                mock_repository_result: Err(InsertNewUserError::UserExists),
                expected_result: Err(UserRegistrationError::UserAlreadyRegistered),
            },
            TestCase {
                description: "When a user is successfully registered, I should receive no error",
                new_user: UserAccountDTO {
                    full_name: "".to_owned(),
                    email: "".to_owned(),
                    role: "".to_owned(),
                    username: "".to_owned(),
                    password: "".to_owned(),
                },
                mock_repository_result: Ok(()),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            let mut mock_repository = MockUserRepository::new();
            eprintln!("{}", test_case.description);
            mock_repository
                .expect_insert_new_user()
                .return_once(|_| test_case.mock_repository_result);
            let registration_service = new_user_registration_service(
                new_hashing_service(HashingProvider::Bcrypt),
                Arc::new(mock_repository),
            );
            let actual_result = registration_service.register_user(test_case.new_user).await;
            assert!(matches!(test_case.expected_result, actual_result));
        }
    }
}
