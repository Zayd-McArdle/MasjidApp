use crate::shared::payment::service::PaymentService;
use std::sync::Arc;

pub struct DonationServiceImpl<R>
where
    R: Send + Sync + ?Sized,
{
    pub payment_service: Arc<dyn PaymentService>,
    pub repository: Arc<R>,
    pub in_memory_repository: Arc<R>,
}
