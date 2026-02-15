use crate::shared::services::email::r#trait::EmailService;
use crate::shared::services::payment::service::PaymentService;
use std::sync::Arc;

pub struct DonationServiceImpl<R>
where
    R: Send + Sync + ?Sized,
{
    pub payment_service: Arc<dyn PaymentService>,
    pub email_service: Arc<dyn EmailService>,
    pub repository: Arc<R>,
    pub in_memory_repository: Arc<R>,
}
