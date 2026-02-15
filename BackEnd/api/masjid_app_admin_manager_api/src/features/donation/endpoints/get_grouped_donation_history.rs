use crate::features::donation::errors::grouped_donation_history_error::GroupedDonationHistoryError;
use crate::features::donation::models::donation_dto::DonationHistoryDTO;
use crate::features::donation::models::get_donation_history_with_grouping_request::GetDonationHistoryWithGroupingRequest;
use crate::features::donation::services::DonationHistoryService;
use crate::shared::jwt::Claims;
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use masjid_app_api_library::shared::http_responses::bad_request::bad_request;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::collections::HashMap;
use std::sync::Arc;
const NO_GROUPED_DONATION_TRANSACTIONS_FOUND_RESPONSE_MESSAGE: &'static str =
    "No grouped donation transactions found.";
const UNABLE_TO_FETCH_GROUPED_TRANSACTION_RECORDS_RESPONSE_MESSAGE: &'static str =
    "Unable to fetch grouped donation transactions at this time. Please try again later.";
pub async fn get_grouped_donation_history(
    State(app_state): State<ServiceAppState<Arc<dyn DonationHistoryService>>>,
    _claims: Claims,
    Query(request): Query<GetDonationHistoryWithGroupingRequest>,
) -> Result<Json<HashMap<String, Vec<DonationHistoryDTO>>>, (StatusCode, String)> {
    let grouped_donation_history = app_state
        .service
        .get_grouped_donation_transaction_history(request.try_into().map_err(bad_request)?)
        .await
        .map_err(|err| match err {
            GroupedDonationHistoryError::UnableToGetGroupedDonationHistory => (
                StatusCode::INTERNAL_SERVER_ERROR,
                UNABLE_TO_FETCH_GROUPED_TRANSACTION_RECORDS_RESPONSE_MESSAGE.to_owned(),
            ),
        })?;

    if grouped_donation_history.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            NO_GROUPED_DONATION_TRANSACTIONS_FOUND_RESPONSE_MESSAGE.to_owned(),
        ));
    }
    Ok(Json(grouped_donation_history))
}
