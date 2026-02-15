use crate::features::donation::errors::grouped_donation_history_error::GroupedDonationHistoryError;
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

const NO_GROUPED_DONATION_TRANSACTION_COUNT_FOUND_RESPONSE_MESSAGE: &'static str =
    "No grouped donation transaction count found.";
const UNABLE_TO_FETCH_GROUPED_TRANSACTION_COUNT_RESPONSE_MESSAGE: &'static str =
    "Unable to fetch grouped donation transaction count at this time. Please try again later.";

pub async fn get_grouped_donation_history_count(
    State(app_state): State<ServiceAppState<Arc<dyn DonationHistoryService>>>,
    _claims: Claims,
    Query(request): Query<GetDonationHistoryWithGroupingRequest>,
) -> Result<Json<HashMap<String, i64>>, (StatusCode, String)> {
    let donation_history_count = app_state
        .service
        .get_grouped_donation_transaction_history_count(request.try_into().map_err(bad_request)?)
        .await
        .map_err(|err| match err {
            GroupedDonationHistoryError::UnableToGetGroupedDonationHistory => (
                StatusCode::INTERNAL_SERVER_ERROR,
                UNABLE_TO_FETCH_GROUPED_TRANSACTION_COUNT_RESPONSE_MESSAGE.to_owned(),
            ),
        })?;
    Ok(Json(donation_history_count))
}
