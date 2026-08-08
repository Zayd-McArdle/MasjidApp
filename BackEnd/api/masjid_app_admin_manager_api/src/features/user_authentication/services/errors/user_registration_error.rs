use crate::features::user_authentication::errors::insert_new_user_error::InsertNewUserError;
use masjid_app_api_library::shared::services::hashing::errors::HashError;

#[derive(Debug)]
pub enum UserRegistrationError {
    UnableToHashPassword(HashError),
    UserAlreadyRegistered,
    UnableToRegisterToRepository,
}

impl From<HashError> for UserRegistrationError {
    #[inline]
    fn from(value: HashError) -> Self {
        Self::UnableToHashPassword(value)
    }
}
impl From<InsertNewUserError> for UserRegistrationError {
    #[inline]
    fn from(value: InsertNewUserError) -> Self {
        match value {
            InsertNewUserError::UserExists => Self::UserAlreadyRegistered,
            InsertNewUserError::DatabaseError => Self::UnableToRegisterToRepository,
        }
    }
}
