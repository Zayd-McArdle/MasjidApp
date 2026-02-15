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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::donation::models::donation_dto::DonationHistoryDTO;
    use crate::features::donation::models::donation_grouping::DonationGrouping;
    use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
    use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;
    use masjid_app_api_library::shared::payment::transaction_status::transaction_declined_reason::TransactionDeclinedReason;
    use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
    use masjid_app_api_library::shared::types::recurrence::Recurrence;

    #[inline]
    fn get_donation_transaction_dtos() -> Vec<DonationHistoryDTO> {
        let donation_transaction_data = include_str!(
            "../../../../../../../../test_data/features/donation/donation_history_data.csv"
        );
        let mut donation_transaction_dtos: Vec<DonationHistoryDTO> = Vec::new();
        let string_to_option_converter = |field| match field {
            "" => None,
            _ => Some(field.to_owned()),
        };
        for line in donation_transaction_data.lines().skip(1) {
            let fields: Vec<&str> = line.split(',').collect();
            donation_transaction_dtos.push(DonationHistoryDTO::from(DonationHistory {
                id: 0,
                cause: fields[0].to_owned(),
                donation_intention: fields[1].to_owned(),
                donor_full_name: fields[2].to_owned(),
                donor_title: fields[3].to_owned(),
                phone_number: fields[4].to_owned(),
                email: string_to_option_converter(fields[5]),
                address_line_1: fields[6].to_owned(),
                address_line_2: string_to_option_converter(fields[7]),
                address_city: fields[8].to_owned(),
                address_region: fields[9].to_owned(),
                address_country: string_to_option_converter(fields[10]),
                address_postal: fields[11].to_owned(),
                amount: fields[12].parse().unwrap(),
                is_gift_aid: fields[13].parse().unwrap(),
                donation_frequency: fields[14].to_owned(),
                transaction_status: fields[15].to_owned(),
            }))
        }
        donation_transaction_dtos
    }
    #[test]
    fn test_group_donation_transaction_history_dtos() {
        struct TestCase {
            donation_transaction_dtos: Vec<DonationHistoryDTO>,
            donation_grouping: DonationGrouping,
            expected_grouped_dtos_len: HashMap<String, usize>,
        }
        let donation_transaction_dtos = get_donation_transaction_dtos();
        let test_cases = [
            TestCase {
                donation_transaction_dtos: donation_transaction_dtos.clone(),
                donation_grouping: DonationGrouping::Cause,
                expected_grouped_dtos_len: HashMap::from([
                    ("Education Fund".to_owned(), 15),
                    ("Emergency Relief".to_owned(), 18),
                    ("Food Program".to_owned(), 12),
                    ("Healthcare Support".to_owned(), 14),
                    ("Mosque Construction".to_owned(), 10),
                    ("Orphan Sponsorship".to_owned(), 10),
                    ("Water Well Project".to_owned(), 11),
                    ("Winter Appeal".to_owned(), 10),
                ]),
            },
            TestCase {
                donation_transaction_dtos: donation_transaction_dtos.clone(),
                donation_grouping: DonationGrouping::Intention,
                expected_grouped_dtos_len: HashMap::from([
                    (DonationIntention::Lillah.to_string(), 32),
                    (DonationIntention::Sadaqah.to_string(), 34),
                    (DonationIntention::Zakat.to_string(), 34),
                ]),
            },
            TestCase {
                donation_transaction_dtos: donation_transaction_dtos.clone(),
                donation_grouping: DonationGrouping::Amount,
                expected_grouped_dtos_len: HashMap::from([
                    ("55".to_owned(), 1),
                    ("60".to_owned(), 1),
                    ("65".to_owned(), 1),
                    ("70".to_owned(), 1),
                    ("75".to_owned(), 1),
                    ("80".to_owned(), 1),
                    ("85".to_owned(), 1),
                    ("90".to_owned(), 2),
                    ("95".to_owned(), 1),
                    ("100".to_owned(), 1),
                    ("105".to_owned(), 1),
                    ("110".to_owned(), 1),
                    ("115".to_owned(), 1),
                    ("120".to_owned(), 1),
                    ("125".to_owned(), 1),
                    ("130".to_owned(), 1),
                    ("135".to_owned(), 1),
                    ("140".to_owned(), 1),
                    ("145".to_owned(), 1),
                    ("150".to_owned(), 2),
                    ("160".to_owned(), 2),
                    ("170".to_owned(), 1),
                    ("180".to_owned(), 2),
                    ("190".to_owned(), 2),
                    ("200".to_owned(), 1),
                    ("210".to_owned(), 1),
                    ("220".to_owned(), 1),
                    ("230".to_owned(), 1),
                    ("240".to_owned(), 1),
                    ("250".to_owned(), 1),
                    ("270".to_owned(), 1),
                    ("300".to_owned(), 2),
                    ("330".to_owned(), 1),
                    ("340".to_owned(), 1),
                    ("350".to_owned(), 1),
                    ("360".to_owned(), 1),
                    ("370".to_owned(), 1),
                    ("380".to_owned(), 2),
                    ("390".to_owned(), 1),
                    ("400".to_owned(), 1),
                    ("410".to_owned(), 1),
                    ("420".to_owned(), 1),
                    ("440".to_owned(), 1),
                    ("450".to_owned(), 2),
                    ("470".to_owned(), 1),
                    ("490".to_owned(), 1),
                    ("500".to_owned(), 1),
                    ("510".to_owned(), 1),
                    ("520".to_owned(), 1),
                    ("530".to_owned(), 1),
                    ("550".to_owned(), 1),
                    ("560".to_owned(), 1),
                    ("580".to_owned(), 1),
                    ("600".to_owned(), 1),
                    ("650".to_owned(), 1),
                    ("670".to_owned(), 1),
                    ("700".to_owned(), 1),
                    ("720".to_owned(), 1),
                    ("740".to_owned(), 1),
                    ("750".to_owned(), 1),
                    ("760".to_owned(), 1),
                    ("800".to_owned(), 1),
                    ("840".to_owned(), 1),
                    ("850".to_owned(), 1),
                    ("870".to_owned(), 1),
                    ("880".to_owned(), 1),
                    ("890".to_owned(), 1),
                    ("900".to_owned(), 1),
                    ("920".to_owned(), 1),
                    ("950".to_owned(), 1),
                    ("1000".to_owned(), 1),
                    ("1200".to_owned(), 1),
                    ("1300".to_owned(), 1),
                    ("1400".to_owned(), 1),
                    ("1500".to_owned(), 1),
                    ("1600".to_owned(), 1),
                    ("1700".to_owned(), 1),
                    ("1800".to_owned(), 1),
                    ("1900".to_owned(), 1),
                    ("2000".to_owned(), 1),
                    ("2200".to_owned(), 1),
                    ("2500".to_owned(), 1),
                    ("2600".to_owned(), 1),
                    ("2800".to_owned(), 1),
                    ("2900".to_owned(), 1),
                    ("3000".to_owned(), 1),
                    ("3100".to_owned(), 1),
                    ("3300".to_owned(), 1),
                    ("3500".to_owned(), 1),
                    ("3800".to_owned(), 1),
                    ("4200".to_owned(), 1),
                    ("5000".to_owned(), 1),
                ]),
            },
            TestCase {
                donation_transaction_dtos: donation_transaction_dtos.clone(),
                donation_grouping: DonationGrouping::GiftAid,
                expected_grouped_dtos_len: HashMap::from([
                    (false.to_string(), 38),
                    (true.to_string(), 62),
                ]),
            },
            TestCase {
                donation_transaction_dtos: donation_transaction_dtos.clone(),
                donation_grouping: DonationGrouping::Frequency,
                expected_grouped_dtos_len: HashMap::from([
                    (Recurrence::OneOff.to_string(), 28),
                    (Recurrence::Daily.to_string(), 0),
                    (Recurrence::Weekly.to_string(), 23),
                    (Recurrence::Fortnightly.to_string(), 0),
                    (Recurrence::Monthly.to_string(), 36),
                    (Recurrence::Annually.to_string(), 13),
                ]),
            },
            TestCase {
                donation_transaction_dtos: donation_transaction_dtos.clone(),
                donation_grouping: DonationGrouping::TransactionStatus,
                expected_grouped_dtos_len: HashMap::from([
                    (TransactionStatus::Approved.to_string(), 60),
                    (
                        TransactionStatus::Declined(TransactionDeclinedReason::CardExpired)
                            .to_string(),
                        10,
                    ),
                    (
                        TransactionStatus::Declined(TransactionDeclinedReason::InsufficientFunds)
                            .to_string(),
                        20,
                    ),
                    (
                        TransactionStatus::Declined(TransactionDeclinedReason::CardFrozen)
                            .to_string(),
                        0,
                    ),
                    (
                        TransactionStatus::Declined(TransactionDeclinedReason::SuspectedFraud)
                            .to_string(),
                        10,
                    ),
                ]),
            },
        ];
        for test_case in test_cases {
            let actual_grouped_dtos = group_donation_transaction_history_dtos(
                test_case.donation_transaction_dtos,
                &test_case.donation_grouping,
            );
            for (key, value) in test_case.expected_grouped_dtos_len {
                if actual_grouped_dtos.contains_key(&key) {
                    assert_eq!(actual_grouped_dtos[&key].len(), value);
                }
            }
        }
    }
}
