use crate::shared::services::payment::billing_address::BillingAddress;
use crate::shared::services::payment::card_details::CardDetails;
use crate::shared::services::payment::errors::PaymentServiceError;
use crate::shared::services::payment::service::PaymentService;
use crate::shared::services::payment::transaction_status::r#enum::TransactionStatus;
use crate::shared::types::recurrence::Recurrence;
use async_trait::async_trait;

pub(in crate::shared::services::payment) struct StripePaymentService;
#[async_trait]
impl PaymentService for StripePaymentService {
    async fn pay(
        &self,
        amount: u32,
        card_details: &CardDetails,
        address: &BillingAddress,
    ) -> Result<TransactionStatus, PaymentServiceError> {
        todo!()
    }
    async fn refund(
        &self,
        amount: u32,
        card_details: &CardDetails,
        address: &BillingAddress,
    ) -> Result<TransactionStatus, PaymentServiceError> {
        todo!()
    }
    async fn pay_subscription(
        &self,
        amount: u32,
        card_details: &CardDetails,
        address: &BillingAddress,
        recurrence: Recurrence,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<TransactionStatus, PaymentServiceError> {
        todo!()
    }

    async fn cancel_subscription(&self, subscription_id: &u64) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_stripe_payment_service_pay() {}
    #[tokio::test]
    async fn test_stripe_payment_service_pay_subscription() {}
    #[tokio::test]
    async fn test_stripe_payment_service_cancel_subscription() {}
}
