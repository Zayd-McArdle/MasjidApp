use crate::common::data_access_layer::mysql::setup_container;
use crate::common::data_access_layer::DatabaseCredentials;
use crate::common::logging::setup_logging;
use masjid_app_admin_manager_api::features::donation::errors::donation_history_admin_repository_errors::GetDonationTransactionsError;
use masjid_app_admin_manager_api::features::donation::models::donation_dto::DonationHistoryDTO;
use masjid_app_admin_manager_api::features::donation::models::donation_filter::DonationFilter;
use masjid_app_admin_manager_api::features::donation::repositories::new_donation_history_admin_repository;
use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;
use masjid_app_api_library::shared::data_access::repository_manager::RepositoryMode;
use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
use masjid_app_api_library::shared::types::personal_title::PersonalTitle;
use masjid_app_api_library::shared::types::recurrence::Recurrence;
use masjid_app_public_api::features::donation::errors::InsertDonationTransactionError;
use masjid_app_public_api::features::donation::repositories::new_donation_history_public_repository;

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

    let mut donation_filter = DonationFilter::default();
    eprintln!("When retrieving donations from an empty database, I should receive an error");
    let get_donation_transaction_result = admin_repository
        .get_donation_transactions(&donation_filter)
        .await
        .unwrap_err();
    assert!(matches!(
        get_donation_transaction_result,
        GetDonationTransactionsError::NotFound
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
        let insert_donation_result = public_repository
            .insert_donation_transaction(donation_transaction)
            .await;
        assert!(insert_donation_result.is_ok());
    }
    eprintln!(
        "When retrieving donations from the database without filters, I should get all donations without error"
    );
    let mut donation_transaction_dtos = admin_repository
        .get_donation_transactions(&donation_filter)
        .await
        .unwrap();
    assert_eq!(donation_transactions.len(), donation_transaction_dtos.len());

    eprintln!(
        "When receiving donations from the database by donation cause, I should get donations filtered by specific cause without error"
    );
    const DONATION_CAUSE: &'static str = "water well";
    donation_filter.donation_cause = Some(DONATION_CAUSE.to_owned());
    donation_transaction_dtos = admin_repository
        .get_donation_transactions(&donation_filter)
        .await
        .unwrap();
    let donation_transactions_by_cause: Vec<&DonationHistory> = donation_transactions
        .iter()
        .filter(|donation_transaction| donation_transaction.cause == DONATION_CAUSE)
        .collect();
    assert_dto(&donation_transaction_dtos, donation_transactions_by_cause);

    eprintln!(
        "When receiving donations from the database by intention, I should get donations filtered by specific intention without error"
    );
    donation_filter.donation_intention = Some(DonationIntention::Lillah);
    donation_transaction_dtos = admin_repository
        .get_donation_transactions(&donation_filter)
        .await
        .unwrap();
    let donation_by_intention: Vec<&DonationHistory> = donation_transactions
        .iter()
        .filter(|donation_transaction| {
            donation_transaction.donation_intention == DonationIntention::Lillah.to_string()
        })
        .collect();
    assert_dto(&donation_transaction_dtos, donation_by_intention);

    eprintln!(
        "When receiving donations from the database by donation amount, I should get donations filtered by specific amount without error"
    );
    donation_filter = DonationFilter::default();
    donation_filter.amount = Some(12000);
    donation_transaction_dtos = admin_repository
        .get_donation_transactions(&donation_filter)
        .await
        .unwrap();
    let donation_transactions_by_amount: Vec<&DonationHistory> = donation_transactions
        .iter()
        .filter(|donation_transaction| donation_transaction.amount == 12000)
        .collect();
    assert_dto(&donation_transaction_dtos, donation_transactions_by_amount);
    container.stop().await.unwrap();
}
#[inline]
fn assert_dto(
    donation_transaction_dtos: &Vec<DonationHistoryDTO>,
    donation_transactions_by_cause: Vec<&DonationHistory>,
) {
    assert_eq!(
        donation_transaction_dtos.len(),
        donation_transactions_by_cause.len()
    );
    for i in 0..donation_transactions_by_cause.len() {
        assert_eq!(
            donation_transactions_by_cause[i].cause,
            donation_transaction_dtos[i].donation_details.cause
        );
        assert_eq!(
            donation_transactions_by_cause[i].donation_intention,
            donation_transaction_dtos[i]
                .donation_details
                .donation_intention
                .to_string()
        );
        assert_eq!(
            donation_transactions_by_cause[i].donor_full_name,
            donation_transaction_dtos[i]
                .donation_details
                .contact_details
                .full_name
        );
        assert_eq!(
            donation_transactions_by_cause[i].donor_title,
            donation_transaction_dtos[i]
                .donation_details
                .contact_details
                .title
                .unwrap()
                .to_string()
        );
        assert_eq!(
            donation_transactions_by_cause[i].phone_number,
            donation_transaction_dtos[i]
                .donation_details
                .contact_details
                .phone_number
        );
        assert_eq!(
            donation_transactions_by_cause[i].email,
            donation_transaction_dtos[i]
                .donation_details
                .contact_details
                .email
        );
        assert_eq!(
            donation_transactions_by_cause[i].amount,
            donation_transaction_dtos[i].donation_details.amount
        );
        assert_eq!(
            donation_transactions_by_cause[i].is_gift_aid,
            donation_transaction_dtos[i].donation_details.is_gift_aid
        );
        assert_eq!(
            donation_transactions_by_cause[i].donation_frequency,
            donation_transaction_dtos[i]
                .donation_details
                .donation_frequency
                .to_string()
        );
        assert_eq!(
            donation_transactions_by_cause[i].transaction_status,
            donation_transaction_dtos[i].transaction_status.to_string()
        );
    }
}

#[inline]
fn get_donation_transactions() -> Vec<DonationHistory> {
    vec![
        DonationHistory {
            id: 0,
            cause: "masjid expansion".to_owned(),
            donation_intention: DonationIntention::Sadaqah.to_string(),
            donor_full_name: "Zayd McArdle".to_owned(),
            donor_title: PersonalTitle::Mr.to_string(),
            phone_number: "07712 345678".to_owned(),
            email: None,
            address_line_1: "999 Test Street".to_owned(),
            address_line_2: None,
            address_city: "Test City".to_owned(),
            address_region: "Test Region".to_owned(),
            address_country: None,
            address_postal: "TT5 5TT".to_owned(),
            amount: 100,
            is_gift_aid: false,
            donation_frequency: Recurrence::OneOff.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "masjid expansion".to_owned(),
            donation_intention: DonationIntention::Zakat.to_string(),
            donor_full_name: "Zayd McArdle".to_owned(),
            donor_title: PersonalTitle::Mr.to_string(),
            phone_number: "07712 345678".to_owned(),
            email: None,
            address_line_1: "999 Test Street".to_owned(),
            address_line_2: None,
            address_city: "Test City".to_owned(),
            address_region: "Test Region".to_owned(),
            address_country: None,
            address_postal: "TT5 5TT".to_owned(),
            amount: 1000,
            is_gift_aid: false,
            donation_frequency: Recurrence::OneOff.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "masjid expansion".to_owned(),
            donation_intention: DonationIntention::Sadaqah.to_string(),
            donor_full_name: "Zayd McArdle".to_owned(),
            donor_title: PersonalTitle::Mr.to_string(),
            phone_number: "07712 345678".to_owned(),
            email: Some("test@test.com".to_owned()),
            address_line_1: "999 Test Street".to_owned(),
            address_line_2: None,
            address_city: "Test City".to_owned(),
            address_region: "Test Region".to_owned(),
            address_country: None,
            address_postal: "TT5 5TT".to_owned(),
            amount: 12000,
            is_gift_aid: false,
            donation_frequency: Recurrence::OneOff.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "water well".to_owned(),
            donation_intention: DonationIntention::Sadaqah.to_string(),
            donor_full_name: "Zayd McArdle".to_owned(),
            donor_title: PersonalTitle::Mr.to_string(),
            phone_number: "07712 345678".to_owned(),
            email: None,
            address_line_1: "999 Test Street".to_owned(),
            address_line_2: None,
            address_city: "Test City".to_owned(),
            address_region: "Test Region".to_owned(),
            address_country: None,
            address_postal: "TT5 5TT".to_owned(),
            amount: 10,
            is_gift_aid: false,
            donation_frequency: Recurrence::OneOff.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "water well".to_owned(),
            donation_intention: DonationIntention::Lillah.to_string(),
            donor_full_name: "Zayd McArdle".to_owned(),
            donor_title: PersonalTitle::Mr.to_string(),
            phone_number: "07712 345678".to_owned(),
            email: None,
            address_line_1: "999 Test Street".to_owned(),
            address_line_2: None,
            address_city: "Test City".to_owned(),
            address_region: "Test Region".to_owned(),
            address_country: None,
            address_postal: "TT5 5TT".to_owned(),
            amount: 10,
            is_gift_aid: false,
            donation_frequency: Recurrence::OneOff.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
    ]
}
