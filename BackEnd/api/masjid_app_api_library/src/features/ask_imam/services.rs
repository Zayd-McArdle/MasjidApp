use std::sync::Arc;

pub struct AskImamServiceImpl<R>
where
    R: Send + Sync + ?Sized,
{
    pub repository: Arc<R>,
    pub in_memory_repository: Arc<R>,
}
