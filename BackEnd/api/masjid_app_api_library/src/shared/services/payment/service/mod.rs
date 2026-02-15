pub mod factory;
pub mod payment_service_provider;
mod stripe_impl;

use crate::shared::services::payment::billing_address::BillingAddress;
use crate::shared::services::payment::card_details::CardDetails;
use crate::shared::services::payment::errors::PaymentServiceError;
use crate::shared::services::payment::transaction_status::r#enum::TransactionStatus;
use crate::shared::types::recurrence::Recurrence;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait PaymentService: Send + Sync {
    async fn pay(
        &self,
        amount: u32,
        card_details: &CardDetails,
        address: &BillingAddress,
    ) -> Result<TransactionStatus, PaymentServiceError>;
    async fn refund(
        &self,
        amount: u32,
        card_details: &CardDetails,
        address: &BillingAddress,
    ) -> Result<TransactionStatus, PaymentServiceError>;
    async fn pay_subscription(
        &self,
        amount: u32,
        card_details: &CardDetails,
        address: &BillingAddress,
        recurrence: Recurrence,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<TransactionStatus, PaymentServiceError>;
    async fn cancel_subscription(&self, subscription_id: &u64);
}
