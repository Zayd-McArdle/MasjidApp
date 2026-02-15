use crate::features::donation::models::donation_dto::DonationHistoryDTO;
use crate::features::donation::models::donation_grouping::DonationGrouping;
use masjid_app_api_library::shared::data_access::db_retrieval_mode::DBRetrievalMode;
use std::collections::HashMap;

/// Returns a group by stored procedure if only the count is returned. Otherwise, an order by stored procedure gets returned
#[inline]
pub(super) const fn get_stored_procedure_internal(
    db_retrieval_mode: DBRetrievalMode,
    count_result: &'static str,
    non_count_result: &'static str,
) -> &'static str {
    match db_retrieval_mode {
        DBRetrievalMode::Count => count_result,
        DBRetrievalMode::Records => non_count_result,
    }
}

/// Takes a vector of DonationHistoryDTO, groups them by a specific field, and then returns a HashMap of that grouping
#[inline]
pub(super) fn group_donation_transaction_history_dtos(
    donation_transaction_dtos: Vec<DonationHistoryDTO>,
    donation_grouping: &DonationGrouping,
) -> HashMap<String, Vec<DonationHistoryDTO>> {
    let mut grouped_transaction_dtos: HashMap<String, Vec<DonationHistoryDTO>> = HashMap::new();
    match donation_grouping {
        DonationGrouping::Cause => donation_transaction_dtos.into_iter().for_each(|dto| {
            grouped_transaction_dtos
                .entry(dto.donation_details.cause.clone())
                .or_insert_with(Vec::new)
                .push(dto);
        }),
        DonationGrouping::Intention => donation_transaction_dtos.into_iter().for_each(|dto| {
            grouped_transaction_dtos
                .entry(dto.donation_details.donation_intention.to_string())
                .or_insert_with(Vec::new)
                .push(dto);
        }),
        DonationGrouping::Amount => donation_transaction_dtos.into_iter().for_each(|dto| {
            grouped_transaction_dtos
                .entry(dto.donation_details.amount.to_string())
                .or_insert_with(Vec::new)
                .push(dto);
        }),
        DonationGrouping::GiftAid => donation_transaction_dtos.into_iter().for_each(|dto| {
            grouped_transaction_dtos
                .entry(dto.donation_details.is_gift_aid.to_string())
                .or_insert_with(Vec::new)
                .push(dto);
        }),
        DonationGrouping::Frequency => donation_transaction_dtos.into_iter().for_each(|dto| {
            grouped_transaction_dtos
                .entry(dto.donation_details.donation_frequency.to_string())
                .or_insert_with(Vec::new)
                .push(dto);
        }),
        DonationGrouping::TransactionStatus => {
            donation_transaction_dtos.into_iter().for_each(|dto| {
                grouped_transaction_dtos
                    .entry(dto.transaction_status.to_string())
                    .or_insert_with(Vec::new)
                    .push(dto);
            })
        }
    }
    grouped_transaction_dtos
}
