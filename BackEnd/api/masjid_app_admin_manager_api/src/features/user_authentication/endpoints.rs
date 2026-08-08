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
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

pub(crate) async fn login(
    State(state): State<ServiceAppState<Arc<dyn LoginService>>>,
    Json(request): Json<LoginRequest>,
) -> Response {
    if let Err(_) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            "The username or password cannot be empty",
        )
            .into_response();
    }

    let login_result = state
        .service
        .login(&request.username, &request.password)
        .await;
    match login_result {
        Ok(role) => {
            let claims = jwt::Claims::generate(&request.username, &role);
            let token_generation_result = jwt::generate_token(&claims);
            if let Ok(token) = token_generation_result {
                return (StatusCode::OK, Json(token)).into_response();
            }
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        Err(LoginError::InvalidCredentials) => StatusCode::UNAUTHORIZED.into_response(),
        Err(LoginError::UnableToLogin) | Err(LoginError::UnableToVerifyPasswordHash) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub(crate) async fn register_user(
    State(state): State<ServiceAppState<Arc<dyn UserRegistrationService>>>,
    Json(request): Json<RegistrationRequest>,
) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let new_user = UserAccountDTO {
        full_name: request.full_name,
        email: request.email,
        role: request.role,
        username: request.username,
        password: request.password,
    };
    match state.service.register_user(new_user).await {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(UserRegistrationError::UserAlreadyRegistered) => StatusCode::CONFLICT.into_response(),
        Err(UserRegistrationError::UnableToRegisterToRepository)
        | Err(UserRegistrationError::UnableToHashPassword(_)) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub(crate) async fn reset_user_password(
    State(state): State<ServiceAppState<Arc<dyn ResetPasswordService>>>,
    Json(request): Json<ResetUserPasswordRequest>,
) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    match state
        .service
        .reset_password(&request.username, &request.replacement_password)
        .await
    {
        Ok(()) => StatusCode::OK.into_response(),
        Err(ResetPasswordError::UserDoesNotExist) => StatusCode::NOT_FOUND.into_response(),
        Err(ResetPasswordError::UnableToResetPassword)
        | Err(ResetPasswordError::UnableToHashPassword(_)) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::user_authentication::services::login_service::MockLoginService;
    use crate::features::user_authentication::services::reset_password_service::MockResetPasswordService;
    use crate::features::user_authentication::services::user_registration_service::MockUserRegistrationService;

    #[derive(Clone)]
    struct TestCase<TRequest, TOk, TErr> {
        description: &'static str,
        request: TRequest,
        expected_service_response: Option<Result<TOk, TErr>>,
        expected_status_code: StatusCode,
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
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "Given the request body is valid but unable to validate login credentials, I should get an INTERNAL_SERVER_ERROR",
                request: valid_request.clone(),
                expected_service_response: Some(Err(LoginError::UnableToLogin)),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "Given the request body is valid but login credentials are invalid, I should get an UNAUTHORIZED response",
                request: valid_request.clone(),
                expected_service_response: Some(Err(LoginError::InvalidCredentials)),
                expected_status_code: StatusCode::UNAUTHORIZED,
            },
            TestCase {
                description: "Given the request body is valid and when database successfully validates credentials, I should get an OK response",
                request: valid_request.clone(),
                expected_service_response: Some(Ok("Admin".to_owned())),
                expected_status_code: StatusCode::OK,
            },
            TestCase {
                description: "Given the request body is valid and when database successfully validates credentials, I should get an OK response",
                request: valid_request.clone(),
                expected_service_response: Some(Ok("Imam".to_owned())),
                expected_status_code: StatusCode::OK,
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
            let actual_response = login(State(app_state), Json(test_case.request)).await;
            assert_eq!(test_case.expected_status_code, actual_response.status());
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
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "Given the request body is valid but registration fails, I should get an INTERNAL_SERVER_ERROR",
                request: valid_request.clone(),
                expected_service_response: Some(Err(
                    UserRegistrationError::UnableToRegisterToRepository,
                )),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "Given the request body is valid but the user already exists, I should get a CONFLICT response",
                request: valid_request.clone(),
                expected_service_response: Some(Err(UserRegistrationError::UserAlreadyRegistered)),
                expected_status_code: StatusCode::CONFLICT,
            },
            TestCase {
                description: "Given the request body is valid and registration succeeds, I should get a CREATED response",
                request: valid_request.clone(),
                expected_service_response: Some(Ok(())),
                expected_status_code: StatusCode::CREATED,
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
            let actual_response = register_user(State(app_state), Json(test_case.request)).await;
            assert_eq!(test_case.expected_status_code, actual_response.status());
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
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "Given the request body is valid but password reset fails, I should get an INTERNAL_SERVER_ERROR",
                request: valid_request.clone(),
                expected_service_response: Some(Err(ResetPasswordError::UnableToResetPassword)),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "Given the request body is valid but the user does not exist, I should get a NOT_FOUND response",
                request: valid_request.clone(),
                expected_service_response: Some(Err(ResetPasswordError::UserDoesNotExist)),
                expected_status_code: StatusCode::NOT_FOUND,
            },
            TestCase {
                description: "Given the request body is valid and password reset succeeds, I should get an OK response",
                request: valid_request.clone(),
                expected_service_response: Some(Ok(())),
                expected_status_code: StatusCode::OK,
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
            let actual_response =
                reset_user_password(State(app_state), Json(test_case.request)).await;
            assert!(matches!(test_case.expected_status_code, actual_resposne));
        }
    }
}
