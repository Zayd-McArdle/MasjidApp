use std::sync::Arc;

pub struct CommonServiceImpl<R>
where
    R: Send + Sync + ?Sized,
{
    pub in_memory_repository: Arc<R>,
    pub repository: Arc<R>,
}