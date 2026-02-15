use crate::shared::payment::billing_address::BillingAddress;
use crate::shared::payment::card_details::CardDetails;
use crate::shared::payment::errors::PaymentServiceError;
use crate::shared::payment::transaction_status::TransactionStatus;
use crate::shared::types::recurrence::Recurrence;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait PaymentService: Send + Sync {
    async fn pay(
        &self,
        card_details: &CardDetails,
        address: &BillingAddress,
    ) -> Result<TransactionStatus, PaymentServiceError>;
    async fn pay_subscription(
        &self,
        card_details: &CardDetails,
        address: &BillingAddress,
        recurrence: Recurrence,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<TransactionStatus, PaymentServiceError>;
    async fn cancel_subscription(&self, subscription_id: &u64);
}

pub struct StripePaymentService;
#[async_trait]
impl PaymentService for StripePaymentService {
    async fn pay(
        &self,
        card_details: &CardDetails,
        address: &BillingAddress,
    ) -> Result<TransactionStatus, PaymentServiceError> {
        todo!()
    }
    async fn pay_subscription(
        &self,
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
