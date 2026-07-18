use std::sync::Arc;

pub struct CommonServiceImpl<R>
where
    R: Send + Sync + ?Sized,
{
    pub in_memory_repository: Arc<R>,
    pub repository: Arc<R>,
}

#[macro_export]
macro_rules! new_service {
    ($service_name: ident, $service_trait: ident, $repository_trait: ident) => {
        pub fn $service_name(
            repository: Arc<dyn $repository_trait>,
            in_memory_repository: Arc<dyn $repository_trait>,
        ) -> Arc<dyn $service_trait> {
            Arc::new(CommonServiceImpl {
                repository,
                in_memory_repository,
            })
        }
    };
}
