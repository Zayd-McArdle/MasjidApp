use crate::features::user_authentication::repositories::UserRepository;
use masjid_app_api_library::shared::services::hashing::r#trait::HashingService;
use std::sync::Arc;

pub(super) struct AuthenticationServiceImpl {
    pub(super) hashing_service: Arc<dyn HashingService>,
    pub(super) user_repository: Arc<dyn UserRepository>,
}
