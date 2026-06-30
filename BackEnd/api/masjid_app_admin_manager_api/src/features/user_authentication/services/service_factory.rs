#[macro_export]
macro_rules! new_authentication_service {
    ($service_name: ident, $trait_name: ident) => {
        pub fn $service_name(
            hashing_service: Arc<dyn HashingService>,
            user_repository: Arc<dyn UserRepository>,
        ) -> Arc<dyn $trait_name> {
            Arc::new(AuthenticationServiceImpl {
                hashing_service,
                user_repository,
            })
        }
    };
}
