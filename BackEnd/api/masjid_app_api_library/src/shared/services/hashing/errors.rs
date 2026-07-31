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
    #[inline]
    fn from(value: BcryptError) -> Self {
        match value {
            BcryptError::CostNotAllowed(..)
            | BcryptError::InvalidHash(_)
            | BcryptError::Rand(_) => Self::UnknownError,
            BcryptError::Truncation(size) => Self::InputTooLarge(size),
        }
    }
}

impl From<BcryptError> for VerifyHashError {
    #[inline]
    fn from(value: BcryptError) -> Self {
        match value {
            BcryptError::CostNotAllowed(_) | BcryptError::Rand(_) => Self::UnknownError,
            BcryptError::InvalidHash(_) => Self::HashMalformed,
            BcryptError::Truncation(hash_size) => Self::InputTooLarge(hash_size),
        }
    }
}
