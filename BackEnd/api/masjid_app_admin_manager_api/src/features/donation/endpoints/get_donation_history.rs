use crate::features::donation::errors::donation_history_service_errors::GetDonationTransactionHistoryError;
use crate::features::donation::models::donation_dto::DonationHistoryDTO;
use crate::features::donation::models::get_donation_history_request::GetDonationHistoryRequest;
use crate::features::donation::services::DonationHistoryService;
use crate::shared::jwt::Claims;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use masjid_app_api_library::shared::http_responses::bad_request::bad_request;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

const NO_DONATION_TRANSACTIONS_FOUND_RESPONSE_MESSAGE: &'static str =
    "No donation transactions found.";
const UNABLE_TO_FETCH_TRANSACTION_RECORDS_RESPONSE_MESSAGE: &'static str =
    "Unable to fetch donation transactions at this time. Please try again later.";

pub async fn get_donation_history(
    State(app_state): State<ServiceAppState<Arc<dyn DonationHistoryService>>>,
    _claims: Claims,
    Query(request): Query<GetDonationHistoryRequest>,
) -> Result<Json<Vec<DonationHistoryDTO>>, (StatusCode, String)> {
    let donation_history = app_state
        .service
        .get_donation_transaction_history(request.try_into().map_err(bad_request)?)
        .await
        .map_err(|err| match err {
            GetDonationTransactionHistoryError::UnableToFetchRecordsFromRepository => (
                StatusCode::INTERNAL_SERVER_ERROR,
                UNABLE_TO_FETCH_TRANSACTION_RECORDS_RESPONSE_MESSAGE.to_owned(),
            ),
        })?;
    if donation_history.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            NO_DONATION_TRANSACTIONS_FOUND_RESPONSE_MESSAGE.to_owned(),
        ));
    }
    Ok(Json(donation_history))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::donation::services::MockDonationHistoryService;
    use masjid_app_api_library::features::donation::models::donation_details::DonationDetails;
    use masjid_app_api_library::shared::payment::billing_address::BillingAddress;
    use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;

    #[tokio::test]
    async fn test_get_donation_history() {
        struct TestCase {
            description: &'static str,
            request: GetDonationHistoryRequest,
            expected_service_result:
                Option<Result<Vec<DonationHistoryDTO>, GetDonationTransactionHistoryError>>,
            expected_result: Result<Vec<DonationHistoryDTO>, (StatusCode, String)>,
        }
        let donation_transactions = vec![DonationHistoryDTO {
            id: 1,
            donation_details: DonationDetails::default(),
            donor_address: BillingAddress::default(),
            transaction_status: TransactionStatus::Approved,
        }];
        let test_cases = [
            TestCase {
                description: "When an invalid request is received, I should get a BAD_REQUEST response",
                request: GetDonationHistoryRequest {
                    email: Some("invalid email".to_owned()),
                    donation_intention: None,
                    phone_number: None,
                    amount: None,
                    is_gift_aid: None,
                    donation_frequency: None,
                    donation_cause: None,
                    transaction_status: None,
                },
                expected_service_result: None,
                expected_result: Err((StatusCode::BAD_REQUEST, String::new())),
            },
            TestCase {
                description: "When no donation transactions found, I should get a NOT_FOUND response",
                request: GetDonationHistoryRequest::default(),
                expected_service_result: Some(Ok(vec![])),
                expected_result: Err((
                    StatusCode::NOT_FOUND,
                    NO_DONATION_TRANSACTIONS_FOUND_RESPONSE_MESSAGE.to_owned(),
                )),
            },
            TestCase {
                description: "When donation transactions could not be retrieved, I should get an INTERNAL_SERVER_ERROR response",
                request: GetDonationHistoryRequest::default(),
                expected_service_result: Some(Err(
                    GetDonationTransactionHistoryError::UnableToFetchRecordsFromRepository,
                )),
                expected_result: Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    UNABLE_TO_FETCH_TRANSACTION_RECORDS_RESPONSE_MESSAGE.to_owned(),
                )),
            },
            TestCase {
                description: "When donations transactions are successfully retrieved, I should get an OK response",
                request: GetDonationHistoryRequest::default(),
                expected_service_result: Some(Ok(donation_transactions.clone())),
                expected_result: Ok(donation_transactions),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_donation_service = MockDonationHistoryService::new();
            if let Some(expected_service_result) = test_case.expected_service_result {
                mock_donation_service
                    .expect_get_donation_transaction_history()
                    .return_once(move |_| expected_service_result);
            }

            let is_validated_request = test_case.request.validate().is_ok();
            let app_state = ServiceAppState::<Arc<dyn DonationHistoryService>> {
                service: Arc::new(mock_donation_service),
            };
            let actual_response = get_donation_history(
                State(app_state),
                Claims::default(),
                Query(test_case.request),
            )
            .await;
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
