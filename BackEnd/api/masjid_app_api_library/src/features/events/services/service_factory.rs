#[macro_export]
macro_rules! new_event_service {
    ($service_name: ident, $service_trait: ident, $repository_trait: ident) => {
        #[inline]
        pub fn $service_name(
            repository: Arc<dyn $repository_trait>,
            in_memory_repository: Arc<dyn $repository_trait>,
        ) -> Arc<dyn $service_trait> {
            Arc::new(EventServiceImpl {
                common: CommonServiceImpl::<dyn $repository_trait> {
                    repository,
                    in_memory_repository,
                },
            })
        }
    };
}
