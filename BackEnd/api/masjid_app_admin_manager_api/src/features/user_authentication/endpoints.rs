use crate::features::user_authentication::errors::{
    LoginError, RegistrationError, ResetPasswordError,
};
use crate::features::user_authentication::models::{
    LoginRequest, RegistrationRequest, ResetUserPasswordRequest, UserAccountDTO,
};
use crate::features::user_authentication::repository::UserRepository;
use crate::shared::jwt;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::types::app_state::AppState;
use std::sync::Arc;
use validator::Validate;

pub(crate) async fn login(
    State(state): State<AppState<Arc<dyn UserRepository>>>,
    Json(request): Json<LoginRequest>,
) -> Response {
    if let Err(_) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            "The username or password cannot be empty",
        )
            .into_response();
    }
    let mut login_result = state
        .repository_map
        .get(&DbType::MySql)
        .unwrap()
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
        Err(LoginError::UnableToLogin) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub(crate) async fn register_user(
    State(state): State<AppState<Arc<dyn UserRepository>>>,
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
    let mut register_user_result = state
        .repository_map
        .get(&DbType::MySql)
        .unwrap()
        .register_user(new_user.clone())
        .await;
    match register_user_result {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(RegistrationError::UserAlreadyRegistered) => StatusCode::CONFLICT.into_response(),
        Err(RegistrationError::FailedToRegister) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub(crate) async fn reset_user_password(
    State(state): State<AppState<Arc<dyn UserRepository>>>,
    Json(request): Json<ResetUserPasswordRequest>,
) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let mut password_reset_result = state
        .repository_map
        .get(&DbType::MySql)
        .unwrap()
        .reset_user_password(&request.username, &request.replacement_password)
        .await;
    match password_reset_result {
        Ok(()) => StatusCode::OK.into_response(),
        Err(ResetPasswordError::UserDoesNotExist) => StatusCode::NOT_FOUND.into_response(),
        Err(ResetPasswordError::FailedToResetUserPassword) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::user_authentication::repository::MockUserRepository;
    use std::collections::HashMap;

    #[derive(Clone)]
    struct TestCase<TRequest, TOk, TErr> {
        request: TRequest,
        expected_db_response: Option<Result<TOk, TErr>>,
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
            //Given the request body is empty, I should receive a BAD_REQUEST
            TestCase {
                request: LoginRequest {
                    username: "".to_string(),
                    password: "".to_string(),
                },
                expected_db_response: None,
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            //Given the request body is valid but unable to validate login credentials, I should get an INTERNAL_SERVER_ERROR
            TestCase {
                request: valid_request.clone(),
                expected_db_response: Some(Err(LoginError::UnableToLogin)),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            //Given the request body is valid but login credentials are invalid, I should get an UNAUTHORIZED response
            TestCase {
                request: valid_request.clone(),
                expected_db_response: Some(Err(LoginError::InvalidCredentials)),
                expected_status_code: StatusCode::UNAUTHORIZED,
            },
            //Given the request body is valid and when database successfully validates credentials, I should get an OK response
            TestCase {
                request: valid_request.clone(),
                expected_db_response: Some(Ok("Admin".to_owned())),
                expected_status_code: StatusCode::OK,
            },
            TestCase {
                request: valid_request.clone(),
                expected_db_response: Some(Ok("Imam".to_owned())),
                expected_status_code: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            let mut mock_repository = MockUserRepository::new();

            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_repository
                    .expect_login()
                    .returning(move |username, password| expected_db_response.clone());
            }
            let arc_repository: Arc<dyn UserRepository> = Arc::new(mock_repository);
            let repository_map: HashMap<DbType, Arc<dyn UserRepository>> =
                HashMap::from([(DbType::MySql, arc_repository)]);
            let app_state: AppState<Arc<dyn UserRepository>> = AppState { repository_map };
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
                request: RegistrationRequest {
                    full_name: "".to_string(),
                    email: "".to_string(),
                    role: "".to_string(),
                    username: "".to_string(),
                    password: "".to_string(),
                },
                expected_db_response: None,
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                request: valid_request.clone(),
                expected_db_response: Some(Err(RegistrationError::FailedToRegister)),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                request: valid_request.clone(),
                expected_db_response: Some(Err(RegistrationError::UserAlreadyRegistered)),
                expected_status_code: StatusCode::CONFLICT,
            },
            TestCase {
                request: valid_request.clone(),
                expected_db_response: Some(Ok(())),
                expected_status_code: StatusCode::CREATED,
            },
        ];
        for test_case in test_cases {
            let mut mock_user_repository = MockUserRepository::new();
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_user_repository
                    .expect_register_user()
                    .returning(move |dto| expected_db_response.clone());
            }
            let arc_repository: Arc<dyn UserRepository> = Arc::new(mock_user_repository);
            let app_state: AppState<Arc<dyn UserRepository>> = AppState {
                repository_map: HashMap::from([(DbType::MySql, arc_repository)]),
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
                request: ResetUserPasswordRequest {
                    username: "".to_string(),
                    replacement_password: "".to_string(),
                },
                expected_db_response: None,
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                request: valid_request.clone(),
                expected_db_response: Some(Err(ResetPasswordError::FailedToResetUserPassword)),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                request: valid_request.clone(),
                expected_db_response: Some(Err(ResetPasswordError::UserDoesNotExist)),
                expected_status_code: StatusCode::NOT_FOUND,
            },
            TestCase {
                request: valid_request.clone(),
                expected_db_response: Some(Ok(())),
                expected_status_code: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            let mut mock_user_repository = MockUserRepository::new();
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_user_repository
                    .expect_reset_user_password()
                    .returning(move |username, password| expected_db_response.clone());
            }
            let arc_repository: Arc<dyn UserRepository> = Arc::new(mock_user_repository);
            let app_state: AppState<Arc<dyn UserRepository>> = AppState {
                repository_map: HashMap::from([(DbType::MySql, arc_repository)]),
            };
            let actual_response = reset_user_password(State(app_state), Json(test_case.request));
        }
    }
}
