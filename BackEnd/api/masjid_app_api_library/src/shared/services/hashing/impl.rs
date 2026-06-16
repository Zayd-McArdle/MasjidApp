use crate::shared::services::hashing::errors::{HashError, VerifyHashError};
use crate::shared::services::hashing::r#trait::HashingService;
use bcrypt::{BcryptError, BcryptResult};

pub(super) struct HashingServiceImpl;
impl HashingService for HashingServiceImpl {
    fn hash(&self, input: &[u8]) -> Result<String, HashError> {
        bcrypt::hash(input, 12).map_err(|err| {
            if let BcryptError::Truncation(input_len) = err {
                return HashError::InputTooLarge(input_len);
            }
            tracing::error!(error = ?err, "Hashing failed");
            HashError::UnknownError
        })
    }

    fn verify_hash(&self, input: &[u8], expected_hash: &str) -> Result<bool, VerifyHashError> {
        bcrypt::verify(input, expected_hash).map_err(VerifyHashError::from)
    }
}
