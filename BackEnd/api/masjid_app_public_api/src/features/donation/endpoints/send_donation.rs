use crate::features::donation::errors::SendDonationServiceError;
use crate::features::donation::models::send_donation_request::SendDonationRequest;
use crate::features::donation::models::send_donation_response::SendDonationResponse;
use crate::features::donation::services::DonationPublicService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use masjid_app_api_library::shared::http_responses::bad_request::bad_request;
use masjid_app_api_library::shared::payment::errors::PaymentServiceError;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

const SERVICE_UNAVAILABLE_RESPONSE_MESSAGE: &str =
    "Access to payment gateway not available at the moment. Aborting payment.";
const INTERNAL_SERVER_ERROR_RESPONSE_MESSAGE: &str = "Internal server error.";
pub async fn send_donation(
    State(app_state): State<ServiceAppState<Arc<dyn DonationPublicService>>>,
    Json(request): Json<SendDonationRequest>,
) -> Result<Json<SendDonationResponse>, (StatusCode, String)> {
    tracing::debug!("incoming donation request: {:?}", &request);
    request.validate().map_err(bad_request)?;
    let transaction_status = app_state
        .service
        .send_donation(
            request.donation_details,
            request.card_details,
            request.billing_address,
        )
        .await
        .map_err(|err| match err {
            SendDonationServiceError::PaymentServiceError(
                PaymentServiceError::ServiceUnavailable,
            ) => (
                StatusCode::SERVICE_UNAVAILABLE,
                SERVICE_UNAVAILABLE_RESPONSE_MESSAGE.to_owned(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                INTERNAL_SERVER_ERROR_RESPONSE_MESSAGE.to_owned(),
            ),
        })?;
    Ok(Json(SendDonationResponse::from(transaction_status)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::donation::errors::{
        InsertDonationTransactionError, SendDonationServiceError,
    };
    use crate::features::donation::services::{DonationPublicService, MockDonationPublicService};
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::Json;
    use masjid_app_api_library::features::donation::models::donation_details::DonationDetails;
    use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;
    use masjid_app_api_library::shared::payment::billing_address::BillingAddress;
    use masjid_app_api_library::shared::payment::card_details::{CardDetails, ExpiryDate};
    use masjid_app_api_library::shared::payment::errors::PaymentServiceError;
    use masjid_app_api_library::shared::payment::transaction_status::transaction_declined_reason::TransactionDeclinedReason;
    use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
    use masjid_app_api_library::shared::types::app_state::ServiceAppState;
    use masjid_app_api_library::shared::types::contact_details::ContactDetails;
    use masjid_app_api_library::shared::types::recurrence::Recurrence;
    use std::sync::Arc;
    use validator::Validate;

    #[tokio::test]
    async fn test_send_donation() {
        struct TestCase {
            description: &'static str,
            request: SendDonationRequest,
            expected_service_response: Option<Result<TransactionStatus, SendDonationServiceError>>,
            expected_result: Result<Json<SendDonationResponse>, (StatusCode, String)>,
        }
        let valid_request = SendDonationRequest {
            donation_details: DonationDetails {
                cause: "Mosque Donation".to_string(),
                donation_intention: DonationIntention::default(),
                is_gift_aid: false,
                contact_details: ContactDetails {
                    full_name: "Zayd McArdle".to_string(),
                    title: None,
                    phone_number: "07123456789".to_string(),
                    email: Some("zaydmcardle@masjidapp.com".to_owned()),
                },
                amount: 100,
                donation_frequency: Recurrence::OneOff,
            },
            card_details: CardDetails {
                cardholder_name: "Zayd McArdle".to_string(),
                card_number: "4539571147647251".to_string(),
                cvv: "123".to_string(),
                expiry_date: ExpiryDate {
                    month: 12,
                    year: 99,
                },
            },
            billing_address: BillingAddress {
                line_1: "124 Test Street".to_string(),
                line_2: None,
                city: "Wingleton".to_string(),
                region: "London".to_string(),
                country: None,
                postal_code: "SW99 ZSL".to_string(),
            },
        };
        let test_cases = [
            TestCase {
                description: "When an invalid request is received, I should get a BAD_REQUEST response",
                request: SendDonationRequest {
                    donation_details: DonationDetails {
                        cause: "".to_string(),
                        donation_intention: DonationIntention::default(),
                        is_gift_aid: false,
                        contact_details: ContactDetails {
                            full_name: "".to_string(),
                            title: None,
                            phone_number: "".to_string(),
                            email: None,
                        },
                        amount: 0,
                        donation_frequency: Recurrence::OneOff,
                    },
                    card_details: CardDetails {
                        cardholder_name: "".to_string(),
                        card_number: "".to_string(),
                        cvv: "".to_string(),
                        expiry_date: ExpiryDate { month: 0, year: 0 },
                    },
                    billing_address: BillingAddress {
                        line_1: "".to_string(),
                        line_2: None,
                        city: "".to_string(),
                        region: "".to_string(),
                        country: None,
                        postal_code: "".to_string(),
                    },
                },
                expected_service_response: None,
                expected_result: Err((StatusCode::BAD_REQUEST, "".to_owned())),
            },
            TestCase {
                description: "When sending donation fails due to payment gateway error, I should get a SERVICE_UNAVAILABLE response",
                request: valid_request.clone(),
                expected_service_response: Some(Err(
                    SendDonationServiceError::PaymentServiceError(
                        PaymentServiceError::ServiceUnavailable,
                    ),
                )),
                expected_result: Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    SERVICE_UNAVAILABLE_RESPONSE_MESSAGE.to_owned(),
                )),
            },
            TestCase {
                description: "When sending donation fails for internal reasons, I should get an INTERNAL_SERVER_ERROR response",
                request: valid_request.clone(),
                expected_service_response: Some(Err(
                    SendDonationServiceError::DonationHistoryPublicRepositoryError(
                        InsertDonationTransactionError::UnableToInsertTransaction,
                    ),
                )),
                expected_result: Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    INTERNAL_SERVER_ERROR_RESPONSE_MESSAGE.to_owned(),
                )),
            },
            TestCase {
                description: "When sending donation returns a declined transaction, I should get an OK response with decline reason",
                request: valid_request.clone(),
                expected_service_response: Some(Ok(TransactionStatus::Declined(
                    TransactionDeclinedReason::CardExpired,
                ))),
                expected_result: Ok(Json(SendDonationResponse {
                    transaction_status: TransactionStatus::Declined(
                        TransactionDeclinedReason::CardExpired,
                    )
                    .to_string(),
                })),
            },
            TestCase {
                description: "When sending donation returns an approved transaction, I should get an OK response",
                request: valid_request,
                expected_service_response: Some(Ok(TransactionStatus::Approved)),
                expected_result: Ok(Json(SendDonationResponse {
                    transaction_status: TransactionStatus::Approved.to_string(),
                })),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_donation_service = MockDonationPublicService::new();
            if let Some(expected_service_response) = test_case.expected_service_response {
                mock_donation_service
                    .expect_send_donation()
                    .returning(move |_, _, _| expected_service_response);
            }
            let arc_service: Arc<dyn DonationPublicService> = Arc::new(mock_donation_service);
            let app_state = ServiceAppState {
                service: arc_service,
            };
            let is_validated_request = test_case.request.validate().is_ok();
            let actual_response = send_donation(State(app_state), Json(test_case.request)).await;
            if is_validated_request {
                assert!(matches!(test_case.expected_result, actual_response));
            } else {
                assert_eq!(
                    test_case.expected_result.unwrap_err().0,
                    actual_response.unwrap_err().0
                );
            }
        }
    }
}
