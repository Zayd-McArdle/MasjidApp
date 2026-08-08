use crate::features::user_authentication::repositories::UserRepository;
use crate::features::user_authentication::services::authentication_service_impl::AuthenticationServiceImpl;
use crate::features::user_authentication::services::errors::login_error::LoginError;
use async_trait::async_trait;
use masjid_app_api_library::shared::services::hashing::r#trait::HashingService;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait LoginService: Send + Sync {
    async fn login(&self, username: &str, password: &str) -> Result<String, LoginError>;
}
#[async_trait]
impl LoginService for AuthenticationServiceImpl {
    async fn login(&self, username: &str, password: &str) -> Result<String, LoginError> {
        let user = self
            .user_repository
            .get_user_by_credentials(username, password)
            .await
            .map_err(LoginError::from)?;
        let hash_verified = self
            .hashing_service
            .verify_hash(password.as_bytes(), &user.password)
            .map_err(|_| LoginError::UnableToVerifyPasswordHash)?;
        if hash_verified {
            tracing::info!(username = username, "logged in");
            return Ok(user.role);
        }

        Err(LoginError::InvalidCredentials)
    }
}

pub fn new_login_service(
    hashing_service: Arc<dyn HashingService>,
    user_repository: Arc<dyn UserRepository>,
) -> Arc<dyn LoginService> {
    Arc::new(AuthenticationServiceImpl {
        hashing_service,
        user_repository,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::user_authentication::errors::get_user_error::GetUserError;
    use crate::features::user_authentication::models::login_dto::LoginDTO;
    use crate::features::user_authentication::repositories::MockUserRepository;
    use crate::features::user_authentication::services::errors::login_error::LoginError;
    use masjid_app_api_library::shared::services::hashing::errors::VerifyHashError;
    use masjid_app_api_library::shared::services::hashing::r#trait::MockHashingService;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_login_service_login() {
        struct TestCase {
            description: &'static str,
            mock_repository_result: Result<LoginDTO, GetUserError>,
            mock_hashing_service_result: Option<Result<bool, VerifyHashError>>,
            expected_result: Result<String, LoginError>,
        }
        let mock_dto = LoginDTO {
            username: "user".to_owned(),
            password: "123".to_owned(),
            role: "admin".to_owned(),
        };
        let test_cases = [
            TestCase {
                description: "When repository receive a database error, an unable to login error should occur",
                mock_repository_result: Err(GetUserError::DatabaseError),
                mock_hashing_service_result: None,
                expected_result: Err(LoginError::UnableToLogin),
            },
            TestCase {
                description: "When repository cannot find user, an invalid credentials error should occur",
                mock_repository_result: Err(GetUserError::NotFound),
                mock_hashing_service_result: None,
                expected_result: Err(LoginError::InvalidCredentials),
            },
            TestCase {
                description: "When repository finds a user but the stored hash is malformed, a hashing error should occur",
                mock_repository_result: Ok(mock_dto.clone()),
                mock_hashing_service_result: Some(Err(VerifyHashError::HashMalformed)),
                expected_result: Err(LoginError::UnableToVerifyPasswordHash),
            },
            TestCase {
                description: "When repository finds a user but password does not match stored hash, an invalid credentials error should occur",
                mock_repository_result: Ok(mock_dto.clone()),
                mock_hashing_service_result: Some(Ok(false)),
                expected_result: Err(LoginError::InvalidCredentials),
            },
            TestCase {
                description: "When repository finds a user, the specific user role should be returned",
                mock_repository_result: Ok(mock_dto.clone()),
                mock_hashing_service_result: Some(Ok(true)),
                expected_result: Ok("admin".to_owned()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_hashing_service = MockHashingService::new();
            let mut mock_repository = MockUserRepository::new();

            if let Some(mock_hashing_service_result) = test_case.mock_hashing_service_result {
                mock_hashing_service
                    .expect_verify_hash()
                    .return_once(|_, _| mock_hashing_service_result);
            }

            mock_repository
                .expect_get_user_by_credentials()
                .return_once(move |_, _| test_case.mock_repository_result);
            let service =
                new_login_service(Arc::new(mock_hashing_service), Arc::new(mock_repository));
            let actual_result = service.login("admin", "password").await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
