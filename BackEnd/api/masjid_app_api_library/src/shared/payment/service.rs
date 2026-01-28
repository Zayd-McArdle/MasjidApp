use crate::shared::payment::billing_address::BillingAddress;
use crate::shared::payment::card_details::CardDetails;
use crate::shared::payment::errors::PaymentServiceError;
use crate::shared::types::recurrence::Recurrence;
use async_trait::async_trait;

#[async_trait]
pub trait PaymentService: Send + Sync {
    async fn pay(
        &self,
        card_details: &CardDetails,
        address: &BillingAddress,
    ) -> Result<(), PaymentServiceError>;
    async fn pay_with_standing_order(
        &self,
        card_details: &CardDetails,
        address: &BillingAddress,
        recurrence: Recurrence,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), PaymentServiceError>;
}

pub struct StripePaymentService;
#[async_trait]
impl PaymentService for StripePaymentService {
    async fn pay(
        &self,
        card_details: &CardDetails,
        address: &BillingAddress,
    ) -> Result<(), PaymentServiceError> {
        todo!()
    }
    async fn pay_with_standing_order(
        &self,
        card_details: &CardDetails,
        address: &BillingAddress,
        recurrence: Recurrence,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), PaymentServiceError> {
        todo!()
    }
}
