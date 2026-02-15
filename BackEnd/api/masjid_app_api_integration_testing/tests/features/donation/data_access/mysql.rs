use crate::common::data_access_layer::mysql::setup_container;
use crate::common::data_access_layer::DatabaseCredentials;
use crate::common::logging::setup_logging;
use masjid_app_admin_manager_api::features::donation::models::donation_dto::DonationHistoryDTO;
use masjid_app_admin_manager_api::features::donation::models::donation_filter::DonationFilter;
use masjid_app_admin_manager_api::features::donation::models::donation_filter_with_grouping::DonationFilterWithGrouping;
use masjid_app_admin_manager_api::features::donation::models::donation_grouping::DonationGrouping;
use masjid_app_admin_manager_api::features::donation::repositories::donation_history_admin_repository::{new_donation_history_admin_repository, DonationHistoryAdminRepository};
use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;
use masjid_app_api_library::shared::data_access::repository_manager::RepositoryMode;
use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
use masjid_app_api_library::shared::traits::value_retriever::ValueRetriever;
use masjid_app_api_library::shared::types::recurrence::Recurrence;
use masjid_app_public_api::features::donation::errors::InsertDonationTransactionError;
use masjid_app_public_api::features::donation::repositories::new_donation_history_public_repository;
use std::sync::Arc;

const DONATION_CAUSE: &'static str = "Water Well Project";

#[tokio::test]
async fn test_donation() {
    setup_logging();
    let container = setup_container(DatabaseCredentials {
        username: "donationadmin".to_owned(),
        password: "changeme".to_owned(),
        environment_variable: "DONATION_CONNECTION".to_owned(),
    })
    .await;

    let public_repository = new_donation_history_public_repository(RepositoryMode::Normal).await;
    let admin_repository = new_donation_history_admin_repository(RepositoryMode::Normal).await;

    eprintln!(
        "When retrieving donations from an empty database, I should receive an empty dataset"
    );
    let _empty_records: Vec<DonationHistoryDTO> = vec![];
    let get_donation_transaction_result = admin_repository
        .get_donation_transactions(&DonationFilter::default())
        .await;
    assert!(matches!(
        get_donation_transaction_result,
        Ok(_empty_records)
    ));

    eprintln!("When inserting an invalid donation transaction, I should receive an error");

    let mut insert_donation_result = public_repository
        .insert_donation_transaction(&DonationHistory {
            id: 0,
            cause: "masjid".to_owned(),
            donation_intention: "".to_owned(),
            donor_full_name: "".to_owned(),
            donor_title: "".to_owned(),
            phone_number: "".to_owned(),
            email: None,
            address_line_1: "".to_owned(),
            address_line_2: None,
            address_city: "".to_owned(),
            address_region: "".to_owned(),
            address_country: None,
            address_postal: "".to_owned(),
            amount: 0,
            is_gift_aid: false,
            donation_frequency: "".to_owned(),
            transaction_status: "".to_owned(),
        })
        .await;
    assert!(matches!(
        insert_donation_result,
        Err(InsertDonationTransactionError::UnableToInsertTransaction)
    ));

    eprintln!("When inserting valid donation transactions, I should get no error");
    let donation_transactions = get_donation_transactions();
    for donation_transaction in donation_transactions.iter() {
        insert_donation_result = public_repository
            .insert_donation_transaction(donation_transaction)
            .await;
        assert!(insert_donation_result.is_ok());
    }

    eprintln!(
        "When retrieving donations from the database without filters, I should get all donations without error"
    );
    let donation_transaction_dtos = admin_repository
        .get_donation_transactions(&DonationFilter::default())
        .await
        .unwrap();
    assert_eq!(donation_transactions.len(), donation_transaction_dtos.len());

    eprintln!(
        "When receiving donations from the database by donation cause, I should get donations filtered by specific cause without error"
    );
    assert_get_donation_transaction_dtos_with_filters(
        &admin_repository,
        DonationFilterSelection::DonationCause(DONATION_CAUSE.to_owned()),
        &donation_transactions,
    )
    .await;

    eprintln!(
        "When receiving donations from the database by intention, I should get donations filtered by specific intention without error"
    );
    assert_get_donation_transaction_dtos_with_filters(
        &admin_repository,
        DonationFilterSelection::DonationIntention(DonationIntention::Lillah),
        &donation_transactions,
    )
    .await;

    eprintln!(
        "When receiving donations from the database by donation amount, I should get donations filtered by specific amount without error"
    );
    assert_get_donation_transaction_dtos_with_filters(
        &admin_repository,
        DonationFilterSelection::Amount(90),
        &donation_transactions,
    )
    .await;

    let groupings = DonationGrouping::get_values();
    let filters = get_donation_filters();

    for grouping in groupings {
        let grouping_str = grouping.to_string();

        eprintln!(
            "When retrieving donation count from the database without filters grouped by {}, I should get count of grouped donations without error",
            grouping_str
        );
        let mut grouped_grouped_donations_history_count = admin_repository
            .get_grouped_donation_transaction_history_count(&DonationFilterWithGrouping {
                filter: DonationFilter::default(),
                donation_grouping: grouping,
            })
            .await;
        assert!(grouped_grouped_donations_history_count.is_ok());

        eprintln!(
            "When retrieving donations from the database without filters grouped by {}, I should get grouped donations without error",
            grouping_str
        );

        let mut grouped_donations_history = admin_repository
            .get_grouped_donation_transaction_history(&DonationFilterWithGrouping {
                filter: DonationFilter::default(),
                donation_grouping: grouping,
            })
            .await;
        assert!(grouped_donations_history.is_ok());

        for filter in filters.clone() {
            eprintln!(
                "When retrieving donation count from the database grouped by {}, I should get count of grouped donations without error\n
                filters: {filter:?}",
                grouping_str
            );
            grouped_grouped_donations_history_count = admin_repository
                .get_grouped_donation_transaction_history_count(&DonationFilterWithGrouping {
                    filter: filter.clone(),
                    donation_grouping: grouping,
                })
                .await;
            assert!(grouped_grouped_donations_history_count.is_ok());

            eprintln!(
                "When retrieving donations from the database grouped by {}, I should get grouped donations without error\n
                filters: {filter:?}",
                grouping_str
            );

            grouped_donations_history = admin_repository
                .get_grouped_donation_transaction_history(&DonationFilterWithGrouping {
                    filter,
                    donation_grouping: grouping,
                })
                .await;
            assert!(grouped_donations_history.is_ok());
        }
    }

    container.stop().await.unwrap();
}

fn get_donation_filters() -> [DonationFilter; 7] {
    [
        DonationFilter {
            donation_cause: None,
            donation_intention: None,
            email: None,
            phone_number: None,
            amount: None,
            is_gift_aid: None,
            donation_frequency: None,
            transaction_status: None,
        },
        DonationFilter {
            donation_cause: Some(DONATION_CAUSE.to_owned()),
            donation_intention: None,
            email: None,
            phone_number: None,
            amount: None,
            is_gift_aid: None,
            donation_frequency: None,
            transaction_status: None,
        },
        DonationFilter {
            donation_cause: None,
            donation_intention: Some(DonationIntention::Lillah),
            email: None,
            phone_number: None,
            amount: None,
            is_gift_aid: None,
            donation_frequency: None,
            transaction_status: None,
        },
        DonationFilter {
            donation_cause: None,
            donation_intention: None,
            email: None,
            phone_number: None,
            amount: Some(75),
            is_gift_aid: None,
            donation_frequency: None,
            transaction_status: None,
        },
        DonationFilter {
            donation_cause: None,
            donation_intention: None,
            email: None,
            phone_number: None,
            amount: None,
            is_gift_aid: Some(true),
            donation_frequency: None,
            transaction_status: None,
        },
        DonationFilter {
            donation_cause: None,
            donation_intention: None,
            email: None,
            phone_number: None,
            amount: None,
            is_gift_aid: Some(true),
            donation_frequency: Some(Recurrence::Monthly),
            transaction_status: None,
        },
        DonationFilter {
            donation_cause: None,
            donation_intention: None,
            email: None,
            phone_number: None,
            amount: None,
            is_gift_aid: None,
            donation_frequency: None,
            transaction_status: Some(TransactionStatus::Approved),
        },
    ]
}

#[derive(Clone)]
enum DonationFilterSelection {
    DonationCause(String),
    DonationIntention(DonationIntention),
    Email(String),
    PhoneNumber(String),
    Amount(u32),
    IsGiftAid(bool),
    DonationFrequency(Recurrence),
    TransactionStatus(TransactionStatus),
}
#[inline]
async fn assert_get_donation_transaction_dtos_with_filters(
    admin_repository: &Arc<dyn DonationHistoryAdminRepository>,
    filter_selection: DonationFilterSelection,
    expected_donation_transactions: &Vec<DonationHistory>,
) {
    let mut donation_filter = DonationFilter::default();
    let expected_filtered_donation_transactions: Vec<&DonationHistory> =
        expected_donation_transactions
            .iter()
            .filter(|donation_transaction| match filter_selection.clone() {
                DonationFilterSelection::DonationCause(cause) => {
                    donation_filter.donation_cause = Some(cause.clone());
                    donation_transaction.cause == cause
                }
                DonationFilterSelection::DonationIntention(intention) => {
                    donation_filter.donation_intention = Some(intention);
                    donation_transaction.donation_intention == intention.to_string()
                }
                DonationFilterSelection::Email(email) => {
                    donation_filter.email = Some(email.clone());
                    donation_transaction.email == Some(email)
                }
                DonationFilterSelection::PhoneNumber(phone_number) => {
                    donation_filter.phone_number = Some(phone_number.clone());
                    donation_transaction.phone_number == phone_number.to_string()
                }
                DonationFilterSelection::Amount(amount) => {
                    donation_filter.amount = Some(amount);
                    donation_transaction.amount == amount
                }
                DonationFilterSelection::IsGiftAid(is_gift_aid) => {
                    donation_filter.is_gift_aid = Some(is_gift_aid);
                    donation_transaction.is_gift_aid == is_gift_aid
                }
                DonationFilterSelection::DonationFrequency(donation_frequency) => {
                    donation_filter.donation_frequency = Some(donation_frequency.clone());
                    donation_transaction.donation_frequency == donation_frequency.to_string()
                }
                DonationFilterSelection::TransactionStatus(status) => {
                    donation_filter.transaction_status = Some(status);
                    donation_transaction.transaction_status == status.to_string()
                }
            })
            .collect();
    let actual_filtered_donation_transaction_dtos = admin_repository
        .get_donation_transactions(&donation_filter)
        .await
        .unwrap();
    assert_dto(
        &actual_filtered_donation_transaction_dtos,
        expected_filtered_donation_transactions,
    );
}

#[inline]
fn assert_dto(
    donation_transaction_dtos: &Vec<DonationHistoryDTO>,
    donation_transactions: Vec<&DonationHistory>,
) {
    assert_eq!(donation_transaction_dtos.len(), donation_transactions.len());
    for i in 0..donation_transactions.len() {
        assert_eq!(
            donation_transactions[i].cause,
            donation_transaction_dtos[i].donation_details.cause
        );
        assert_eq!(
            donation_transactions[i].donation_intention,
            donation_transaction_dtos[i]
                .donation_details
                .donation_intention
                .to_string()
        );
        assert_eq!(
            donation_transactions[i].donor_full_name,
            donation_transaction_dtos[i]
                .donation_details
                .contact_details
                .full_name
        );
        assert_eq!(
            donation_transactions[i].donor_title,
            donation_transaction_dtos[i]
                .donation_details
                .contact_details
                .title
                .unwrap()
                .to_string()
        );
        assert_eq!(
            donation_transactions[i].phone_number,
            donation_transaction_dtos[i]
                .donation_details
                .contact_details
                .phone_number
        );
        assert_eq!(
            donation_transactions[i].email,
            donation_transaction_dtos[i]
                .donation_details
                .contact_details
                .email
        );
        assert_eq!(
            donation_transactions[i].amount,
            donation_transaction_dtos[i].donation_details.amount
        );
        assert_eq!(
            donation_transactions[i].is_gift_aid,
            donation_transaction_dtos[i].donation_details.is_gift_aid
        );
        assert_eq!(
            donation_transactions[i].donation_frequency,
            donation_transaction_dtos[i]
                .donation_details
                .donation_frequency
                .to_string()
        );
        assert_eq!(
            donation_transactions[i].transaction_status,
            donation_transaction_dtos[i].transaction_status.to_string()
        );
    }
}

#[inline]
fn get_donation_transactions() -> Vec<DonationHistory> {
    let donation_transaction_data =
        include_str!("../../../../../../../test_data/features/donation/donation_history_data.csv");
    let mut donation_transactions: Vec<DonationHistory> = Vec::new();
    let string_to_option_converter = |field| match field {
        "" => None,
        _ => Some(field.to_owned()),
    };
    for line in donation_transaction_data.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();
        donation_transactions.push(DonationHistory {
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
        })
    }
    donation_transactions
}
struct RetrievalTestCase {
    description: &'static str,
    filter: Option<(DonationFilter, fn(DonationHistoryDTO) -> bool)>,
}
