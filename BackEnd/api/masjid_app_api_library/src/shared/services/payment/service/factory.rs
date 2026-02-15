use crate::shared::services::payment::service::PaymentService;
use crate::shared::services::payment::service::payment_service_provider::PaymentServiceProvider;
use crate::shared::services::payment::service::stripe_impl::StripePaymentService;
use std::sync::Arc;

pub fn new_payment_service(service_provider: PaymentServiceProvider) -> Arc<dyn PaymentService> {
    match service_provider {
        PaymentServiceProvider::Stripe => Arc::new(StripePaymentService),
    }
}
