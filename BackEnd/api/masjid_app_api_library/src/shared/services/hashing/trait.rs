use crate::shared::services::hashing::errors::{HashError, VerifyHashError};
use crate::shared::services::hashing::providers::HashingProvider;
use crate::shared::services::hashing::r#impl::HashingServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
pub trait HashingService: Send + Sync {
    fn hash(&self, input: &[u8]) -> Result<String, HashError>;
    fn verify_hash(&self, input: &[u8], expected_hash: &str) -> Result<bool, VerifyHashError>;
}

pub fn new_hashing_service(hashing_provider: HashingProvider) -> Arc<dyn HashingService> {
    match hashing_provider {
        HashingProvider::Bcrypt => Arc::new(HashingServiceImpl),
    }
}
