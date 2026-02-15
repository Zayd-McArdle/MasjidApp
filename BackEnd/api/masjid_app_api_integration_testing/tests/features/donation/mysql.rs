use crate::common::data_access_layer::mysql::setup_container;
use crate::common::data_access_layer::DatabaseCredentials;
use crate::common::logging::setup_logging;
use masjid_app_admin_manager_api::features::donation::models::donation_dto::DonationHistoryDTO;
use masjid_app_admin_manager_api::features::donation::models::donation_filter::DonationFilter;
use masjid_app_admin_manager_api::features::donation::repositories::donation_history_admin_repository::new_donation_history_admin_repository;
use masjid_app_api_library::features::donation::models::donation_history::DonationHistory;
use masjid_app_api_library::features::donation::models::donation_intention::DonationIntention;
use masjid_app_api_library::shared::data_access::repository_manager::RepositoryMode;
use masjid_app_api_library::shared::payment::transaction_status::transaction_declined_reason::TransactionDeclinedReason;
use masjid_app_api_library::shared::payment::transaction_status::TransactionStatus;
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
    eprintln!(
        "When retrieving donations from an empty database, I should receive an empty dataset"
    );
    let _empty_records: Vec<DonationHistoryDTO> = vec![];
    let get_donation_transaction_result = admin_repository
        .get_donation_transactions(&donation_filter)
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
    let donation_transaction_data = include_str!("donation_history_data.csv");
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
    vec![
        DonationHistory {
            id: 0,
            cause: "Emergency Relief".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Mr Ahmed Khan".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "07123 456789".to_owned(),
            email: Some("ahmed.khan@email.com".to_owned()),
            address_line_1: "123 High Street".to_owned(),
            address_line_2: None,
            address_city: "London".to_owned(),
            address_region: "Greater London".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "SW1A 1AA".to_owned(),
            amount: 500,
            is_gift_aid: true,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Education Fund".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Mrs Fatima Ali".to_owned(),
            donor_title: "Mrs".to_owned(),
            phone_number: "07123 456790".to_owned(),
            email: Some("fatima.ali@email.com".to_owned()),
            address_line_1: "456 Park Road".to_owned(),
            address_line_2: Some("Flat 3".to_owned()),
            address_city: "Birmingham".to_owned(),
            address_region: "West Midlands".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "B1 1BB".to_owned(),
            amount: 250,
            is_gift_aid: false,
            donation_frequency: "One-time".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Healthcare Support".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Dr Omar Hassan".to_owned(),
            donor_title: "Dr".to_owned(),
            phone_number: "020 1234 5678".to_owned(),
            email: None,
            address_line_1: "789 Queen Street".to_owned(),
            address_line_2: None,
            address_city: "Manchester".to_owned(),
            address_region: "Greater Manchester".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "M1 2CC".to_owned(),
            amount: 1000,
            is_gift_aid: true,
            donation_frequency: "Weekly".to_owned(),
            transaction_status: TransactionStatus::Declined(
                TransactionDeclinedReason::InsufficientFunds,
            )
            .to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Food Program".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Ms Aisha Rahman".to_owned(),
            donor_title: "Ms".to_owned(),
            phone_number: "07123 456791".to_owned(),
            email: Some("aisha.rahman@email.com".to_owned()),
            address_line_1: "321 Church Lane".to_owned(),
            address_line_2: Some("Floor 2".to_owned()),
            address_city: "Leicester".to_owned(),
            address_region: "East Midlands".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "LE1 3DD".to_owned(),
            amount: 75,
            is_gift_aid: true,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Water Well Project".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Prof Yusuf Ahmed".to_owned(),
            donor_title: "Prof".to_owned(),
            phone_number: "020 8765 4321".to_owned(),
            email: Some("yusuf.ahmed@email.com".to_owned()),
            address_line_1: "654 Victoria Road".to_owned(),
            address_line_2: None,
            address_city: "Bradford".to_owned(),
            address_region: "West Yorkshire".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "BD1 4EE".to_owned(),
            amount: 2000,
            is_gift_aid: false,
            donation_frequency: "Annually".to_owned(),
            transaction_status: "Failed".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Orphan Sponsorship".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Mr Bilal Hussain".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "07123 456792".to_owned(),
            email: Some("bilal.hussain@email.com".to_owned()),
            address_line_1: "987 Green Lane".to_owned(),
            address_line_2: None,
            address_city: "Glasgow".to_owned(),
            address_region: "Scotland".to_owned(),
            address_country: Some("Scotland".to_owned()),
            address_postal: "G1 5FF".to_owned(),
            amount: 350,
            is_gift_aid: true,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Mosque Construction".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Mrs Zainab Malik".to_owned(),
            donor_title: "Mrs".to_owned(),
            phone_number: "020 3456 7890".to_owned(),
            email: None,
            address_line_1: "147 Mill Lane".to_owned(),
            address_line_2: Some("Apt 5".to_owned()),
            address_city: "Cardiff".to_owned(),
            address_region: "Wales".to_owned(),
            address_country: Some("Wales".to_owned()),
            address_postal: "CF10 6GG".to_owned(),
            amount: 5000,
            is_gift_aid: true,
            donation_frequency: "One-time".to_owned(),
            transaction_status: "Refunded".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Winter Appeal".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Mr Ali Qureshi".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "07123 456793".to_owned(),
            email: Some("ali.qureshi@email.com".to_owned()),
            address_line_1: "258 Station Road".to_owned(),
            address_line_2: None,
            address_city: "Edinburgh".to_owned(),
            address_region: "Scotland".to_owned(),
            address_country: Some("Scotland".to_owned()),
            address_postal: "EH1 7HH".to_owned(),
            amount: 150,
            is_gift_aid: false,
            donation_frequency: "Weekly".to_owned(),
            transaction_status: "Pending".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Emergency Relief".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Mrs Maryam Begum".to_owned(),
            donor_title: "Mrs".to_owned(),
            phone_number: "020 9876 5432".to_owned(),
            email: Some("maryam.begum@email.com".to_owned()),
            address_line_1: "369 Church Street".to_owned(),
            address_line_2: None,
            address_city: "Liverpool".to_owned(),
            address_region: "Merseyside".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "L1 8II".to_owned(),
            amount: 300,
            is_gift_aid: true,
            donation_frequency: "One-time".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Emergency Relief".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Dr Ibrahim Patel".to_owned(),
            donor_title: "Dr".to_owned(),
            phone_number: "07123 456794".to_owned(),
            email: Some("ibrahim.patel@email.com".to_owned()),
            address_line_1: "741 King Street".to_owned(),
            address_line_2: Some("Suite 1".to_owned()),
            address_city: "Bristol".to_owned(),
            address_region: "South West".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "BS1 9JJ".to_owned(),
            amount: 800,
            is_gift_aid: false,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        // Records 11-20
        DonationHistory {
            id: 0,
            cause: "Education Fund".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Ms Khadija Begum".to_owned(),
            donor_title: "Ms".to_owned(),
            phone_number: "07123 456801".to_owned(),
            email: Some("khadija.begum@email.com".to_owned()),
            address_line_1: "12 Oak Avenue".to_owned(),
            address_line_2: None,
            address_city: "London".to_owned(),
            address_region: "Greater London".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "NW1 2AB".to_owned(),
            amount: 150,
            is_gift_aid: true,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Healthcare Support".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Mr Hassan Malik".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "020 1234 5679".to_owned(),
            email: None,
            address_line_1: "34 Elm Street".to_owned(),
            address_line_2: Some("Garden Flat".to_owned()),
            address_city: "Birmingham".to_owned(),
            address_region: "West Midlands".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "B2 3CD".to_owned(),
            amount: 750,
            is_gift_aid: false,
            donation_frequency: "One-time".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Food Program".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Mrs Safiya Rahman".to_owned(),
            donor_title: "Mrs".to_owned(),
            phone_number: "07123 456802".to_owned(),
            email: Some("safiya.rahman@email.com".to_owned()),
            address_line_1: "56 Maple Road".to_owned(),
            address_line_2: None,
            address_city: "Manchester".to_owned(),
            address_region: "Greater Manchester".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "M2 4EF".to_owned(),
            amount: 120,
            is_gift_aid: true,
            donation_frequency: "Weekly".to_owned(),
            transaction_status: "Pending".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Water Well Project".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Dr Amina Hussain".to_owned(),
            donor_title: "Dr".to_owned(),
            phone_number: "020 8765 4322".to_owned(),
            email: Some("amina.hussain@email.com".to_owned()),
            address_line_1: "78 Cedar Lane".to_owned(),
            address_line_2: None,
            address_city: "Leicester".to_owned(),
            address_region: "East Midlands".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "LE2 5GH".to_owned(),
            amount: 3000,
            is_gift_aid: true,
            donation_frequency: "Annually".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Orphan Sponsorship".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Mr Yusuf Ali".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "07123 456803".to_owned(),
            email: Some("yusuf.ali@email.com".to_owned()),
            address_line_1: "90 Birch Grove".to_owned(),
            address_line_2: Some("Top Floor".to_owned()),
            address_city: "Bradford".to_owned(),
            address_region: "West Yorkshire".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "BD2 6IJ".to_owned(),
            amount: 450,
            is_gift_aid: false,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: "Failed".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Mosque Construction".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Prof Zainab Khan".to_owned(),
            donor_title: "Prof".to_owned(),
            phone_number: "020 3456 7891".to_owned(),
            email: None,
            address_line_1: "23 Willow Way".to_owned(),
            address_line_2: None,
            address_city: "Glasgow".to_owned(),
            address_region: "Scotland".to_owned(),
            address_country: Some("Scotland".to_owned()),
            address_postal: "G2 7KL".to_owned(),
            amount: 3500,
            is_gift_aid: true,
            donation_frequency: "One-time".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Winter Appeal".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Mrs Layla Qureshi".to_owned(),
            donor_title: "Mrs".to_owned(),
            phone_number: "07123 456804".to_owned(),
            email: Some("layla.qureshi@email.com".to_owned()),
            address_line_1: "45 Pine Close".to_owned(),
            address_line_2: None,
            address_city: "Cardiff".to_owned(),
            address_region: "Wales".to_owned(),
            address_country: Some("Wales".to_owned()),
            address_postal: "CF11 8MN".to_owned(),
            amount: 90,
            is_gift_aid: true,
            donation_frequency: "Weekly".to_owned(),
            transaction_status: "Refunded".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Emergency Relief".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Mr Ismail Ahmed".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "020 9876 5433".to_owned(),
            email: Some("ismail.ahmed@email.com".to_owned()),
            address_line_1: "67 Ash Court".to_owned(),
            address_line_2: Some("Flat 8".to_owned()),
            address_city: "Edinburgh".to_owned(),
            address_region: "Scotland".to_owned(),
            address_country: Some("Scotland".to_owned()),
            address_postal: "EH2 9OP".to_owned(),
            amount: 600,
            is_gift_aid: false,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: "Pending".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Education Fund".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Ms Nadia Patel".to_owned(),
            donor_title: "Ms".to_owned(),
            phone_number: "07123 456805".to_owned(),
            email: Some("nadia.patel@email.com".to_owned()),
            address_line_1: "89 Beech Avenue".to_owned(),
            address_line_2: None,
            address_city: "Liverpool".to_owned(),
            address_region: "Merseyside".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "L2 0QR".to_owned(),
            amount: 180,
            is_gift_aid: true,
            donation_frequency: "One-time".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Healthcare Support".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Dr Rashid Mahmood".to_owned(),
            donor_title: "Dr".to_owned(),
            phone_number: "020 1234 5680".to_owned(),
            email: None,
            address_line_1: "12 Sycamore Street".to_owned(),
            address_line_2: None,
            address_city: "Bristol".to_owned(),
            address_region: "South West".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "BS2 1ST".to_owned(),
            amount: 900,
            is_gift_aid: false,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        // Records 21-30 (Emergency Relief focus)
        DonationHistory {
            id: 0,
            cause: "Emergency Relief".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Mr Tariq Bashir".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "07123 456806".to_owned(),
            email: Some("tariq.bashir@email.com".to_owned()),
            address_line_1: "34 Holly Road".to_owned(),
            address_line_2: None,
            address_city: "London".to_owned(),
            address_region: "Greater London".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "SW2 2UV".to_owned(),
            amount: 550,
            is_gift_aid: true,
            donation_frequency: "One-time".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Emergency Relief".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Mrs Samira Akhtar".to_owned(),
            donor_title: "Mrs".to_owned(),
            phone_number: "020 8765 4323".to_owned(),
            email: Some("samira.akhtar@email.com".to_owned()),
            address_line_1: "56 Ivy Lane".to_owned(),
            address_line_2: Some("Flat 12".to_owned()),
            address_city: "Birmingham".to_owned(),
            address_region: "West Midlands".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "B3 3WX".to_owned(),
            amount: 700,
            is_gift_aid: false,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Emergency Relief".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Dr Farooq Siddiqui".to_owned(),
            donor_title: "Dr".to_owned(),
            phone_number: "07123 456807".to_owned(),
            email: None,
            address_line_1: "78 Rowan Gardens".to_owned(),
            address_line_2: None,
            address_city: "Manchester".to_owned(),
            address_region: "Greater Manchester".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "M3 4YZ".to_owned(),
            amount: 450,
            is_gift_aid: true,
            donation_frequency: "Weekly".to_owned(),
            transaction_status: "Pending".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Education Fund".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Ms Rabia Choudhry".to_owned(),
            donor_title: "Ms".to_owned(),
            phone_number: "020 3456 7892".to_owned(),
            email: Some("rabia.choudhry@email.com".to_owned()),
            address_line_1: "90 Laurel Drive".to_owned(),
            address_line_2: None,
            address_city: "Leicester".to_owned(),
            address_region: "East Midlands".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "LE3 5AB".to_owned(),
            amount: 300,
            is_gift_aid: true,
            donation_frequency: "Annually".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Food Program".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Mr Imran Hassan".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "07123 456808".to_owned(),
            email: Some("imran.hassan@email.com".to_owned()),
            address_line_1: "12 Hazel Crescent".to_owned(),
            address_line_2: Some("Basement".to_owned()),
            address_city: "Bradford".to_owned(),
            address_region: "West Yorkshire".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "BD3 6CD".to_owned(),
            amount: 85,
            is_gift_aid: false,
            donation_frequency: "One-time".to_owned(),
            transaction_status: "Failed".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Water Well Project".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Mrs Nasreen Malik".to_owned(),
            donor_title: "Mrs".to_owned(),
            phone_number: "020 9876 5434".to_owned(),
            email: None,
            address_line_1: "34 Elm Gardens".to_owned(),
            address_line_2: None,
            address_city: "Glasgow".to_owned(),
            address_region: "Scotland".to_owned(),
            address_country: Some("Scotland".to_owned()),
            address_postal: "G3 7DE".to_owned(),
            amount: 1500,
            is_gift_aid: true,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Orphan Sponsorship".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Prof Hamza Ali".to_owned(),
            donor_title: "Prof".to_owned(),
            phone_number: "07123 456809".to_owned(),
            email: Some("hamza.ali@email.com".to_owned()),
            address_line_1: "56 Oakfield Road".to_owned(),
            address_line_2: None,
            address_city: "Cardiff".to_owned(),
            address_region: "Wales".to_owned(),
            address_country: Some("Wales".to_owned()),
            address_postal: "CF12 8FG".to_owned(),
            amount: 400,
            is_gift_aid: false,
            donation_frequency: "Weekly".to_owned(),
            transaction_status: "Refunded".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Mosque Construction".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Mrs Salma Begum".to_owned(),
            donor_title: "Mrs".to_owned(),
            phone_number: "020 1234 5681".to_owned(),
            email: Some("salma.begum@email.com".to_owned()),
            address_line_1: "78 Park View".to_owned(),
            address_line_2: Some("Flat 3B".to_owned()),
            address_city: "Edinburgh".to_owned(),
            address_region: "Scotland".to_owned(),
            address_country: Some("Scotland".to_owned()),
            address_postal: "EH3 9GH".to_owned(),
            amount: 2500,
            is_gift_aid: true,
            donation_frequency: "One-time".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Winter Appeal".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Mr Faisal Khan".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "07123 456810".to_owned(),
            email: None,
            address_line_1: "90 Queens Road".to_owned(),
            address_line_2: None,
            address_city: "Liverpool".to_owned(),
            address_region: "Merseyside".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "L3 1HJ".to_owned(),
            amount: 110,
            is_gift_aid: true,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: "Pending".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Emergency Relief".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Dr Asma Hussain".to_owned(),
            donor_title: "Dr".to_owned(),
            phone_number: "020 8765 4324".to_owned(),
            email: Some("asma.hussain@email.com".to_owned()),
            address_line_1: "12 Kings Road".to_owned(),
            address_line_2: None,
            address_city: "Bristol".to_owned(),
            address_region: "South West".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "BS3 2JK".to_owned(),
            amount: 650,
            is_gift_aid: false,
            donation_frequency: "Annually".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        // Records 31-40
        DonationHistory {
            id: 0,
            cause: "Education Fund".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Ms Tahira Rahman".to_owned(),
            donor_title: "Ms".to_owned(),
            phone_number: "07123 456811".to_owned(),
            email: Some("tahira.rahman@email.com".to_owned()),
            address_line_1: "34 Church Road".to_owned(),
            address_line_2: None,
            address_city: "London".to_owned(),
            address_region: "Greater London".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "SW3 3KL".to_owned(),
            amount: 220,
            is_gift_aid: true,
            donation_frequency: "One-time".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Healthcare Support".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Mr Javed Akhtar".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "020 3456 7893".to_owned(),
            email: None,
            address_line_1: "56 Station Road".to_owned(),
            address_line_2: Some("Flat 7".to_owned()),
            address_city: "Birmingham".to_owned(),
            address_region: "West Midlands".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "B4 4MN".to_owned(),
            amount: 850,
            is_gift_aid: false,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Food Program".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Mrs Shabnam Qureshi".to_owned(),
            donor_title: "Mrs".to_owned(),
            phone_number: "07123 456812".to_owned(),
            email: Some("shabnam.qureshi@email.com".to_owned()),
            address_line_1: "78 Victoria Street".to_owned(),
            address_line_2: None,
            address_city: "Manchester".to_owned(),
            address_region: "Greater Manchester".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "M4 5OP".to_owned(),
            amount: 95,
            is_gift_aid: true,
            donation_frequency: "Weekly".to_owned(),
            transaction_status: "Pending".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Water Well Project".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Dr Saeed Mahmood".to_owned(),
            donor_title: "Dr".to_owned(),
            phone_number: "020 9876 5435".to_owned(),
            email: Some("saeed.mahmood@email.com".to_owned()),
            address_line_1: "90 Green Lane".to_owned(),
            address_line_2: None,
            address_city: "Leicester".to_owned(),
            address_region: "East Midlands".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "LE4 6QR".to_owned(),
            amount: 1200,
            is_gift_aid: true,
            donation_frequency: "Annually".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Orphan Sponsorship".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Mr Nadeem Ali".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "07123 456813".to_owned(),
            email: Some("nadeem.ali@email.com".to_owned()),
            address_line_1: "12 Mill Road".to_owned(),
            address_line_2: Some("Ground Floor".to_owned()),
            address_city: "Bradford".to_owned(),
            address_region: "West Yorkshire".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "BD4 7ST".to_owned(),
            amount: 380,
            is_gift_aid: false,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: "Failed".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Mosque Construction".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Prof Ruqayya Khan".to_owned(),
            donor_title: "Prof".to_owned(),
            phone_number: "020 1234 5682".to_owned(),
            email: None,
            address_line_1: "34 Park Lane".to_owned(),
            address_line_2: None,
            address_city: "Glasgow".to_owned(),
            address_region: "Scotland".to_owned(),
            address_country: Some("Scotland".to_owned()),
            address_postal: "G4 8UV".to_owned(),
            amount: 4200,
            is_gift_aid: true,
            donation_frequency: "One-time".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Winter Appeal".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Mrs Haleema Begum".to_owned(),
            donor_title: "Mrs".to_owned(),
            phone_number: "07123 456814".to_owned(),
            email: Some("haleema.begum@email.com".to_owned()),
            address_line_1: "56 King Street".to_owned(),
            address_line_2: None,
            address_city: "Cardiff".to_owned(),
            address_region: "Wales".to_owned(),
            address_country: Some("Wales".to_owned()),
            address_postal: "CF13 9WX".to_owned(),
            amount: 130,
            is_gift_aid: true,
            donation_frequency: "Weekly".to_owned(),
            transaction_status: "Refunded".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Emergency Relief".to_owned(),
            donation_intention: "Zakat".to_owned(),
            donor_full_name: "Mr Zubair Ahmed".to_owned(),
            donor_title: "Mr".to_owned(),
            phone_number: "020 8765 4325".to_owned(),
            email: Some("zubair.ahmed@email.com".to_owned()),
            address_line_1: "78 Queen Street".to_owned(),
            address_line_2: Some("Flat 15".to_owned()),
            address_city: "Edinburgh".to_owned(),
            address_region: "Scotland".to_owned(),
            address_country: Some("Scotland".to_owned()),
            address_postal: "EH4 0YZ".to_owned(),
            amount: 580,
            is_gift_aid: false,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: "Pending".to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "Education Fund".to_owned(),
            donation_intention: "Lillah".to_owned(),
            donor_full_name: "Ms Anisa Patel".to_owned(),
            donor_title: "Ms".to_owned(),
            phone_number: "07123 456815".to_owned(),
            email: Some("anisa.patel@email.com".to_owned()),
            address_line_1: "90 High Road".to_owned(),
            address_line_2: None,
            address_city: "Liverpool".to_owned(),
            address_region: "Merseyside".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "L4 1AB".to_owned(),
            amount: 270,
            is_gift_aid: true,
            donation_frequency: "One-time".to_owned(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
        DonationHistory {
            id: 0,
            cause: "Healthcare Support".to_owned(),
            donation_intention: "Sadaqah".to_owned(),
            donor_full_name: "Dr Kamran Siddiqui".to_owned(),
            donor_title: "Dr".to_owned(),
            phone_number: "020 3456 7894".to_owned(),
            email: None,
            address_line_1: "12 Church Avenue".to_owned(),
            address_line_2: None,
            address_city: "Bristol".to_owned(),
            address_region: "South West".to_owned(),
            address_country: Some("UK".to_owned()),
            address_postal: "BS4 3CD".to_owned(),
            amount: 950,
            is_gift_aid: false,
            donation_frequency: Recurrence::Monthly.to_string(),
            transaction_status: TransactionStatus::Approved.to_string(),
        },
    ]
    /*vec![
        DonationHistory {
            id: 0,
            cause: "masjid expansion".to_owned(),
            donation_intention: DonationIntention::Sadaqah.to_owned(),
            donor_full_name: "Zayd McArdle".to_owned(),
            donor_title: PersonalTitle::Mr.to_owned(),
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
            donation_frequency: Recurrence::OneOff.to_owned(),
            transaction_status: TransactionStatus::Approved.to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "masjid expansion".to_owned(),
            donation_intention: DonationIntention::Zakat.to_owned(),
            donor_full_name: "Zayd McArdle".to_owned(),
            donor_title: PersonalTitle::Mr.to_owned(),
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
            donation_frequency: Recurrence::OneOff.to_owned(),
            transaction_status: TransactionStatus::Approved.to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "masjid expansion".to_owned(),
            donation_intention: DonationIntention::Sadaqah.to_owned(),
            donor_full_name: "Zayd McArdle".to_owned(),
            donor_title: PersonalTitle::Mr.to_owned(),
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
            donation_frequency: Recurrence::OneOff.to_owned(),
            transaction_status: TransactionStatus::Approved.to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "water well".to_owned(),
            donation_intention: DonationIntention::Sadaqah.to_owned(),
            donor_full_name: "Zayd McArdle".to_owned(),
            donor_title: PersonalTitle::Mr.to_owned(),
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
            donation_frequency: Recurrence::OneOff.to_owned(),
            transaction_status: TransactionStatus::Approved.to_owned(),
        },
        DonationHistory {
            id: 0,
            cause: "water well".to_owned(),
            donation_intention: DonationIntention::Lillah.to_owned(),
            donor_full_name: "Zayd McArdle".to_owned(),
            donor_title: PersonalTitle::Mr.to_owned(),
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
            donation_frequency: Recurrence::OneOff.to_owned(),
            transaction_status: TransactionStatus::Approved.to_owned(),
        },
    ]*/
}
