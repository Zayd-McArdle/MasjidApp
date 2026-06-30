use crate::features::user_authentication::errors::GetUserError;

#[derive(Clone, Debug, PartialEq)]
pub enum LoginError {
    InvalidCredentials,
    UnableToVerifyPasswordHash,
    UnableToLogin,
}

impl From<GetUserError> for LoginError {
    fn from(value: GetUserError) -> Self {
        match value {
            GetUserError::NotFound => Self::InvalidCredentials,
            GetUserError::DatabaseError => Self::UnableToLogin,
        }
    }
}
