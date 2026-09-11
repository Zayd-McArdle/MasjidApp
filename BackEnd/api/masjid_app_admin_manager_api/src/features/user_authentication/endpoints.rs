use crate::features::user_authentication::models::login_request::LoginRequest;
use crate::features::user_authentication::models::registration_request::RegistrationRequest;
use crate::features::user_authentication::models::reset_user_password_request::ResetUserPasswordRequest;
use crate::features::user_authentication::models::user_account_dto::UserAccountDTO;
use crate::features::user_authentication::services::errors::login_error::LoginError;
use crate::features::user_authentication::services::errors::reset_password_error::ResetPasswordError;
use crate::features::user_authentication::services::errors::user_registration_error::UserRegistrationError;
use crate::features::user_authentication::services::login_service::LoginService;
use crate::features::user_authentication::services::reset_password_service::ResetPasswordService;
use crate::features::user_authentication::services::user_registration_service::UserRegistrationService;
use crate::shared::jwt;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

pub(crate) async fn login(
    State(state): State<ServiceAppState<Arc<dyn LoginService>>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<String>, StatusCode> {
    request.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    let login_result = state
        .service
        .login(&request.username, &request.password)
        .await;
    match login_result {
        Ok(role) => {
            let claims = jwt::Claims::generate(&request.username, &role);
            let token_generation_result = jwt::generate_token(&claims);
            if let Ok(token) = token_generation_result {
                return Ok(Json(token));
            }
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(LoginError::InvalidCredentials) => Err(StatusCode::UNAUTHORIZED),
        Err(LoginError::UnableToLogin) | Err(LoginError::UnableToVerifyPasswordHash) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub(crate) async fn register_user(
    State(state): State<ServiceAppState<Arc<dyn UserRegistrationService>>>,
    Json(request): Json<RegistrationRequest>,
) -> Result<(), StatusCode> {
    request.validate().map_err(|_| StatusCode::BAD_REQUEST)?;
    let new_user = UserAccountDTO {
        full_name: request.full_name,
        email: request.email,
        role: request.role,
        username: request.username,
        password: request.password,
    };
    state
        .service
        .register_user(new_user)
        .await
        .map_err(move |err| match err {
            UserRegistrationError::UserAlreadyRegistered => StatusCode::CONFLICT,
            UserRegistrationError::UnableToRegisterToRepository
            | UserRegistrationError::UnableToHashPassword(_) => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub(crate) async fn reset_user_password(
    State(state): State<ServiceAppState<Arc<dyn ResetPasswordService>>>,
    Json(request): Json<ResetUserPasswordRequest>,
) -> Result<(), StatusCode> {
    request.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    state
        .service
        .reset_password(&request.username, &request.replacement_password)
        .await
        .map_err(move |err| match err {
            ResetPasswordError::UserDoesNotExist => StatusCode::NOT_FOUND,
            ResetPasswordError::UnableToResetPassword
            | ResetPasswordError::UnableToHashPassword(_) => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::user_authentication::services::login_service::MockLoginService;
    use crate::features::user_authentication::services::reset_password_service::MockResetPasswordService;
    use crate::features::user_authentication::services::user_registration_service::MockUserRegistrationService;

    #[derive(Clone)]
    struct TestCase<TRequest, TOk, TErr, TResponse> {
        description: &'static str,
        request: TRequest,
        expected_service_response: Option<Result<TOk, TErr>>,
        expected_result: TResponse,
    }
    enum ApiType {
        Login,
        Register,
        Reset,
    }

    #[tokio::test]
    async fn test_login() {
        let valid_request = LoginRequest {
            username: "Zayd McArdle".to_owned(),
            password: "Password".to_owned(),
        };
        let test_cases = vec![
            TestCase {
                description: "Given the request body is empty, I should receive a BAD_REQUEST",
                request: LoginRequest {
                    username: "".to_string(),
                    password: "".to_string(),
                },
                expected_service_response: None,
                expected_result: Err(StatusCode::BAD_REQUEST),
            },
            TestCase {
                description: "Given the request body is valid but unable to validate login credentials, I should get an INTERNAL_SERVER_ERROR",
                request: valid_request.clone(),
                expected_service_response: Some(Err(LoginError::UnableToLogin)),
                expected_result: Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            TestCase {
                description: "Given the request body is valid but login credentials are invalid, I should get an UNAUTHORIZED response",
                request: valid_request.clone(),
                expected_service_response: Some(Err(LoginError::InvalidCredentials)),
                expected_result: Err(StatusCode::UNAUTHORIZED),
            },
            TestCase {
                description: "Given the request body is valid and when database successfully validates credentials, I should get an OK response",
                request: valid_request.clone(),
                expected_service_response: Some(Ok("Admin".to_owned())),
                expected_result: Ok(Json("Admin".to_owned())),
            },
            TestCase {
                description: "Given the request body is valid and when database successfully validates credentials, I should get an OK response",
                request: valid_request.clone(),
                expected_service_response: Some(Ok("Imam".to_owned())),
                expected_result: Ok(Json("Imam".to_owned())),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockLoginService::new();

            if let Some(expected_service_response) = test_case.expected_service_response {
                mock_service
                    .expect_login()
                    .return_once(move |_, _| expected_service_response);
            }
            let app_state = ServiceAppState::<Arc<dyn LoginService>> {
                service: Arc::new(mock_service),
            };
            let actual_result = login(State(app_state), Json(test_case.request)).await;
            assert!(match (test_case.expected_result, actual_result) {
                (Ok(_), Ok(_)) => {
                    true
                }
                (Err(expected), Err(actual)) => expected == actual,
                _ => false,
            });
        }
    }

    #[tokio::test]
    async fn test_register_user() {
        let valid_request = RegistrationRequest {
            full_name: "Zayd McArdle".to_string(),
            email: "zaydmcardle@example.com".to_string(),
            role: "Admin".to_string(),
            username: "ZaydMcArdle".to_string(),
            password: "ThisIsMyPasswordForMyUnitTest".to_string(),
        };
        let test_cases = vec![
            TestCase {
                description: "Given the request body is empty, I should receive a BAD_REQUEST",
                request: RegistrationRequest {
                    full_name: "".to_string(),
                    email: "".to_string(),
                    role: "".to_string(),
                    username: "".to_string(),
                    password: "".to_string(),
                },
                expected_service_response: None,
                expected_result: Err(StatusCode::BAD_REQUEST),
            },
            TestCase {
                description: "Given the request body is valid but registration fails, I should get an INTERNAL_SERVER_ERROR",
                request: valid_request.clone(),
                expected_service_response: Some(Err(
                    UserRegistrationError::UnableToRegisterToRepository,
                )),
                expected_result: Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            TestCase {
                description: "Given the request body is valid but the user already exists, I should get a CONFLICT response",
                request: valid_request.clone(),
                expected_service_response: Some(Err(UserRegistrationError::UserAlreadyRegistered)),
                expected_result: Err(StatusCode::CONFLICT),
            },
            TestCase {
                description: "Given the request body is valid and registration succeeds, I should get a CREATED response",
                request: valid_request.clone(),
                expected_service_response: Some(Ok(())),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockUserRegistrationService::new();
            if let Some(expected_service_response) = test_case.expected_service_response {
                mock_service
                    .expect_register_user()
                    .return_once(|_| expected_service_response);
            }
            let arc_service: Arc<dyn UserRegistrationService> = Arc::new(mock_service);
            let app_state = ServiceAppState {
                service: arc_service,
            };
            let actual_result = register_user(State(app_state), Json(test_case.request)).await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }

    #[tokio::test]
    async fn test_reset_user_password() {
        let valid_request = ResetUserPasswordRequest {
            username: "Zayd-McArdle".to_string(),
            replacement_password: "MyReplacementPassword".to_string(),
        };
        let test_cases = vec![
            TestCase {
                description: "Given the request body is empty, I should receive a BAD_REQUEST",
                request: ResetUserPasswordRequest {
                    username: "".to_string(),
                    replacement_password: "".to_string(),
                },
                expected_service_response: None,
                expected_result: Err(StatusCode::BAD_REQUEST),
            },
            TestCase {
                description: "Given the request body is valid but password reset fails, I should get an INTERNAL_SERVER_ERROR",
                request: valid_request.clone(),
                expected_service_response: Some(Err(ResetPasswordError::UnableToResetPassword)),
                expected_result: Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            TestCase {
                description: "Given the request body is valid but the user does not exist, I should get a NOT_FOUND response",
                request: valid_request.clone(),
                expected_service_response: Some(Err(ResetPasswordError::UserDoesNotExist)),
                expected_result: Err(StatusCode::NOT_FOUND),
            },
            TestCase {
                description: "Given the request body is valid and password reset succeeds, I should get an OK response",
                request: valid_request.clone(),
                expected_service_response: Some(Ok(())),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockResetPasswordService::new();
            if let Some(expected_service_response) = test_case.expected_service_response {
                mock_service
                    .expect_reset_password()
                    .return_once(move |_, _| expected_service_response);
            }
            let arc_service: Arc<dyn ResetPasswordService> = Arc::new(mock_service);
            let app_state = ServiceAppState {
                service: arc_service,
            };
            let actual_result =
                reset_user_password(State(app_state), Json(test_case.request)).await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
