use crate::shared::payment::billing::address::Address;
use crate::shared::payment::card_details::CardDetails;
use crate::shared::types::recurrence::Recurrence;
use async_trait::async_trait;

#[async_trait]
pub trait PaymentService {
    async fn pay(&self, card_details: CardDetails, address: Address) -> Result<(), String>;
    async fn pay_with_standing_order(
        &self,
        card_details: CardDetails,
        address: Address,
        recurrence: Recurrence,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), String>;
}

pub struct StripePaymentService;
#[async_trait]
impl PaymentService for StripePaymentService {
    async fn pay(&self, card_details: CardDetails, address: Address) -> Result<(), String> {
        todo!()
    }
    async fn pay_with_standing_order(
        &self,
        card_details: CardDetails,
        address: Address,
        recurrence: Recurrence,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), String> {
        todo!()
    }
}
