use std::sync::Arc;

use crate::shared::common_service_impl::CommonServiceImpl;

pub struct FeedbackServiceImpl<R, S>
where
    R: Send + Sync + ?Sized,
    S: Send + Sync + ?Sized,
{
    pub common: CommonServiceImpl<R>,
    pub image_management_service: Arc<S>,
}
#[macro_export]
macro_rules! new_feedback_service {
    ($service_name: ident, $service_trait: ident, $repository_trait: ident, $image_management_service: ident) => {
        pub fn $service_name(
            repository: Arc<dyn $repository_trait>,
            in_memory_repository: Arc<dyn $repository_trait>,
            image_management_service: Arc<dyn $image_management_service>,
        ) -> Arc<dyn $service_trait> {
            Arc::new(FeedbackServiceImpl {
                common: CommonServiceImpl {
                    repository,
                    in_memory_repository,
                },
                image_management_service,
            })
        }
    };
}
