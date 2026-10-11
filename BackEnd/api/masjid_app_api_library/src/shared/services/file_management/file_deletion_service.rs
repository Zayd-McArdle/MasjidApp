use async_trait::async_trait;

#[async_trait]
pub trait FileDeletionService {
    async fn delete(&self, ids: &[u64]) -> Result<(), ()>;
}
