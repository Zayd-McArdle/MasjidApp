use bcrypt::BcryptError;
#[derive(Debug)]
pub enum HashError {
    InputTooLarge(usize),
    UnknownError,
}

pub enum VerifyHashError {
    InputTooLarge(usize),
    HashMalformed,
    UnknownError,
}

impl From<BcryptError> for HashError {
    fn from(value: BcryptError) -> Self {
        match value {
            BcryptError::CostNotAllowed(..)
            | BcryptError::InvalidHash(_)
            | BcryptError::Rand(_) => HashError::UnknownError,
            BcryptError::Truncation(size) => HashError::InputTooLarge(size),
        }
    }
}

impl From<BcryptError> for VerifyHashError {
    fn from(value: BcryptError) -> Self {
        match value {
            BcryptError::CostNotAllowed(_) | BcryptError::Rand(_) => VerifyHashError::UnknownError,
            BcryptError::InvalidHash(_) => VerifyHashError::HashMalformed,
            BcryptError::Truncation(hash_size) => VerifyHashError::InputTooLarge(hash_size),
        }
    }
}
