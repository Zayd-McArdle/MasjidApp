use crate::features::user_authentication::errors::UpdateUserPasswordError;
use masjid_app_api_library::shared::services::hashing::errors::HashError;

pub enum ResetPasswordError {
    UserDoesNotExist,
    UnableToHashPassword(HashError),
    UnableToResetPassword,
}

impl From<UpdateUserPasswordError> for ResetPasswordError {
    #[inline]
    fn from(value: UpdateUserPasswordError) -> Self {
        match value {
            UpdateUserPasswordError::UserDoesNotExist => Self::UserDoesNotExist,
            UpdateUserPasswordError::DatabaseError => Self::UnableToResetPassword,
        }
    }
}

impl From<HashError> for ResetPasswordError {
    #[inline]
    fn from(value: HashError) -> Self {
        Self::UnableToHashPassword(value)
    }
}
