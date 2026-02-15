use crate::features::donation::errors::SendDonationServiceError;
use crate::features::donation::repositories::DonationHistoryPublicRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::donation::models::donation_details::DonationDetails;
use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
use masjid_app_api_library::features::donation::services::services::DonationServiceImpl;
use masjid_app_api_library::shared::payment::billing_address::BillingAddress;
use masjid_app_api_library::shared::payment::card_details::CardDetails;
use masjid_app_api_library::shared::payment::service::PaymentService;
use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait DonationPublicService: Send + Sync {
    async fn send_donation(
        &self,
        donation_details: DonationDetails,
        card_details: CardDetails,
        billing_address: BillingAddress,
    ) -> Result<TransactionStatus, SendDonationServiceError>;
}

#[async_trait]
impl DonationPublicService for DonationServiceImpl<dyn DonationHistoryPublicRepository> {
    async fn send_donation(
        &self,
        donation_details: DonationDetails,
        card_details: CardDetails,
        billing_address: BillingAddress,
    ) -> Result<TransactionStatus, SendDonationServiceError> {
        let transaction_status = self
            .payment_service
            .pay(&card_details, &billing_address)
            .await
            .map_err(|err| SendDonationServiceError::PaymentServiceError(err))?;

        let donation_transaction_history =
            &DonationHistory::new(donation_details, billing_address, transaction_status);
        // If the payment gateway processes the transaction, store the transaction history in the main database.
        self.repository
            .insert_donation_transaction(donation_transaction_history)
            .await
            .map_err(|err| SendDonationServiceError::DonationHistoryPublicRepositoryError(err))?;

        // If storing the transaction in the main database successful, store it in the in-memory cache as well.
        self.in_memory_repository
            .insert_donation_transaction(donation_transaction_history)
            .await
            .map_err(|err| {
                SendDonationServiceError::DonationHistoryPublicInMemoryRepositoryError(err)
            })?;
        Ok(transaction_status)
    }
}

pub async fn new_donation_public_service(
    payment_service: Arc<dyn PaymentService>,
    repository: Arc<dyn DonationHistoryPublicRepository>,
    in_memory_repository: Arc<dyn DonationHistoryPublicRepository>,
) -> Arc<dyn DonationPublicService> {
    Arc::new(DonationServiceImpl::<dyn DonationHistoryPublicRepository> {
        payment_service,
        repository,
        in_memory_repository,
    })
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::features::donation::errors::InsertDonationTransactionError;
    use crate::features::donation::repositories::MockDonationHistoryPublicRepository;
    use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;
    use masjid_app_api_library::shared::payment::card_details::ExpiryDate;
    use masjid_app_api_library::shared::payment::errors::PaymentServiceError;
    use masjid_app_api_library::shared::payment::service::MockPaymentService;
    use masjid_app_api_library::shared::payment::transaction_status::transaction_declined_reason::TransactionDeclinedReason;
    use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
    use masjid_app_api_library::shared::types::contact_details::ContactDetails;
    use masjid_app_api_library::shared::types::personal_title::PersonalTitle;
    use masjid_app_api_library::shared::types::recurrence::Recurrence;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_donation_public_service_send_donation() {
        struct TestCase {
            description: &'static str,
            donation_details: DonationDetails,
            card_details: CardDetails,
            billing_address: BillingAddress,
            expected_payment_service_result: Result<TransactionStatus, PaymentServiceError>,
            expected_repository_result: Option<Result<(), InsertDonationTransactionError>>,
            expected_in_memory_repository_result:
                Option<Result<(), InsertDonationTransactionError>>,
            expected_result: Result<TransactionStatus, SendDonationServiceError>,
        }

        let donation_details = DonationDetails {
            cause: "mosque donation".to_string(),
            donation_intention: DonationIntention::default(),
            is_gift_aid: false,
            contact_details: ContactDetails {
                full_name: "Zayd McArdle".to_string(),
                title: Some(PersonalTitle::Mr),
                phone_number: "".to_string(),
                email: None,
            },
            amount: 0,
            donation_frequency: Recurrence::OneOff,
        };

        let card_details = CardDetails {
            cardholder_name: "".to_string(),
            card_number: "".to_string(),
            cvv: "".to_string(),
            expiry_date: ExpiryDate { month: 0, year: 0 },
        };

        let billing_address = BillingAddress {
            line_1: "".to_string(),
            line_2: None,
            city: "".to_string(),
            region: "".to_string(),
            country: None,
            postal_code: "".to_string(),
        };

        let test_cases = [
            TestCase {
                description: "When accessing the payment gateway fails, I should get a PaymentServiceError",
                donation_details: donation_details.clone(),
                card_details: card_details.clone(),
                billing_address: billing_address.clone(),
                expected_payment_service_result: Err(PaymentServiceError::ServiceUnavailable),
                expected_repository_result: None,
                expected_in_memory_repository_result: None,
                expected_result: Err(SendDonationServiceError::PaymentServiceError(
                    PaymentServiceError::ServiceUnavailable,
                )),
            },
            TestCase {
                description: "When saving transaction history to main repository fails, I should get DonationHistoryPublicRepositoryError",
                donation_details: donation_details.clone(),
                card_details: card_details.clone(),
                billing_address: billing_address.clone(),
                expected_payment_service_result: Ok(TransactionStatus::Approved),
                expected_repository_result: Some(Err(
                    InsertDonationTransactionError::UnableToInsertTransaction,
                )),
                expected_in_memory_repository_result: None,
                expected_result: Err(
                    SendDonationServiceError::DonationHistoryPublicRepositoryError(
                        InsertDonationTransactionError::UnableToInsertTransaction,
                    ),
                ),
            },
            TestCase {
                description: "When saving transaction history to in-memory repository fails, I should get DonationHistoryPublicRepositoryError",
                donation_details: donation_details.clone(),
                card_details: card_details.clone(),
                billing_address: billing_address.clone(),
                expected_payment_service_result: Ok(TransactionStatus::Approved),
                expected_repository_result: Some(Ok(())),
                expected_in_memory_repository_result: Some(Err(
                    InsertDonationTransactionError::UnableToInsertTransaction,
                )),
                expected_result: Err(
                    SendDonationServiceError::DonationHistoryPublicInMemoryRepositoryError(
                        InsertDonationTransactionError::UnableToInsertTransaction,
                    ),
                ),
            },
            TestCase {
                description: "When payment service declines payment and transaction is stored in all repositories successfully, I should receive no error",
                donation_details: donation_details.clone(),
                card_details: card_details.clone(),
                billing_address: billing_address.clone(),
                expected_payment_service_result: Ok(TransactionStatus::Declined(
                    TransactionDeclinedReason::CardFrozen,
                )),
                expected_repository_result: Some(Ok(())),
                expected_in_memory_repository_result: Some(Ok(())),
                expected_result: Ok(TransactionStatus::Declined(
                    TransactionDeclinedReason::CardFrozen,
                )),
            },
            TestCase {
                description: "When payment service successfully processes payment and transaction is stored in all repositories successfully, I should receive no error",
                donation_details,
                card_details,
                billing_address,
                expected_payment_service_result: Ok(TransactionStatus::Approved),
                expected_repository_result: Some(Ok(())),
                expected_in_memory_repository_result: Some(Ok(())),
                expected_result: Ok(TransactionStatus::Approved),
            },
        ];

        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_payment_service = MockPaymentService::new();
            let mut mock_repository = MockDonationHistoryPublicRepository::new();
            let mut mock_in_memory_repository = MockDonationHistoryPublicRepository::new();

            mock_payment_service
                .expect_pay()
                .returning(move |_, _| test_case.expected_payment_service_result);
            if let Some(expected_repository_result) = test_case.expected_repository_result {
                mock_repository
                    .expect_insert_donation_transaction()
                    .returning(move |_| expected_repository_result);
            }
            if let Some(expected_in_memory_repository_result) =
                test_case.expected_in_memory_repository_result
            {
                mock_in_memory_repository
                    .expect_insert_donation_transaction()
                    .returning(move |_| expected_in_memory_repository_result);
            }

            let donation_service: DonationServiceImpl<dyn DonationHistoryPublicRepository> =
                DonationServiceImpl {
                    payment_service: Arc::new(mock_payment_service),
                    repository: Arc::new(mock_repository),
                    in_memory_repository: Arc::new(mock_in_memory_repository),
                };

            let actual_result = donation_service
                .send_donation(
                    test_case.donation_details,
                    test_case.card_details,
                    test_case.billing_address,
                )
                .await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
