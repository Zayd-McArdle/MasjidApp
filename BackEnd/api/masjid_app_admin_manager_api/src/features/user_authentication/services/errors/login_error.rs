use crate::features::user_authentication::errors::get_user_error::GetUserError;

#[derive(Clone, Debug, PartialEq)]
pub enum LoginError {
    InvalidCredentials,
    UnableToVerifyPasswordHash,
    UnableToLogin,
}

impl From<GetUserError> for LoginError {
    #[inline]
    fn from(value: GetUserError) -> Self {
        match value {
            GetUserError::NotFound => Self::InvalidCredentials,
            GetUserError::DatabaseError => Self::UnableToLogin,
        }
    }
}
