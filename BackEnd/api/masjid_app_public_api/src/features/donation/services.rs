use crate::features::donation::errors::SendDonationServiceError;
use crate::features::donation::repositories::DonationHistoryPublicRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::donation::models::DonationDetails;
use masjid_app_api_library::features::donation::services::DonationServiceImpl;
use masjid_app_api_library::shared::payment::billing_address::BillingAddress;
use masjid_app_api_library::shared::payment::card_details::CardDetails;
use masjid_app_api_library::shared::payment::service::PaymentService;
use mockall::automock;

#[automock]
#[async_trait]
pub trait DonationPublicService: Send + Sync {
    async fn send_donation(
        &self,
        donation_details: DonationDetails,
        card_details: CardDetails,
        billing_address: BillingAddress,
    ) -> Result<(), SendDonationServiceError>;
}

#[async_trait]
impl DonationPublicService for DonationServiceImpl<dyn DonationHistoryPublicRepository> {
    async fn send_donation(
        &self,
        donation_details: DonationDetails,
        card_details: CardDetails,
        billing_address: BillingAddress,
    ) -> Result<(), SendDonationServiceError> {
        self.payment_service
            .pay(&card_details, &billing_address)
            .await
            .map_err(|err| SendDonationServiceError::PaymentServiceError(err))?;
        self.repository.insert_donation_transaction()
        todo!()
    }
}
