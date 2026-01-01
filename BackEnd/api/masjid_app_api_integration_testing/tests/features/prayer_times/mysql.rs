use masjid_app_api_library::features::prayer_times::errors::GetPrayerTimesError;
use masjid_app_api_library::features::prayer_times::models::PrayerTimesDTO;
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_public_api::features::prayer_times::repositories::new_prayer_times_public_repository;
use masjid_app_admin_manager_api::features::prayer_times::repositories::new_prayer_times_admin_repository;
use crate::common::data_access_layer;
use crate::common::data_access_layer::DatabaseCredentials;
use crate::common::logging::setup_logging;
#[tokio::test]
async fn test_prayer_times() {
    setup_logging();
    let container = data_access_layer::mysql::setup_container(DatabaseCredentials {
        username: "prayertimesadmin".to_string(),
        password: "HR0o8NRkwvuMaIBh7yaf".to_string(),
        environment_variable: "PRAYER_TIMES_CONNECTION".to_string(),
    })
        .await;

    let public_repository = new_prayer_times_public_repository(DbType::MySql).await;
    let admin_repository = new_prayer_times_admin_repository().await;

    //Given no prayer times exist, I should receive an error
    let get_prayer_times_result = public_repository.get_prayer_times().await.unwrap_err();
    assert_eq!(
        get_prayer_times_result,
        GetPrayerTimesError::PrayerTimesNotFound
    );

    //When trying to retrieve latest prayer times if none exist, I should receive an error
    let get_updated_prayer_times_result = public_repository
        .get_updated_prayer_times(
            "5a4e9c5d6b8a2f3e1c0b9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d",
        )
        .await
        .unwrap_err();
    assert_eq!(
        get_updated_prayer_times_result,
        GetPrayerTimesError::PrayerTimesNotFound
    );

    //When I insert some prayer times, I should receive no error
    let update_prayer_times_result = admin_repository
        .update_prayer_times(PrayerTimesDTO {
            data: Some(vec![0x01, 0x02, 0x03, 0x04]),
            hash: "5a4e9c5d6b8a2f3e1c0b9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d".to_owned(),
        })
        .await;
    assert!(update_prayer_times_result.is_ok());

    //When I retrieve prayer times, I should receive no error
    let get_prayer_times_result = public_repository.get_prayer_times().await.unwrap();
    assert_eq!(
        get_prayer_times_result.data,
        Some(vec![0x01, 0x02, 0x03, 0x04])
    );

    //When I check for updated prayer times using an old hash, I should receive the latest prayer times and hash with no error
    let get_updated_prayer_times_result = public_repository
        .get_updated_prayer_times(
            "1a3e9c5d6b8a2f3e1c0b9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c6c",
        )
        .await
        .unwrap();
    assert_eq!(
        get_updated_prayer_times_result.data,
        Some(vec![0x01, 0x02, 0x03, 0x04])
    );
    assert_eq!(
        get_updated_prayer_times_result.hash,
        "5a4e9c5d6b8a2f3e1c0b9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d".to_owned()
    );

    //When I check for updated prayer times using the latest hash, I should receive no error
    let get_updated_prayer_times_result = public_repository
        .get_updated_prayer_times(
            "5a4e9c5d6b8a2f3e1c0b9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d",
        )
        .await
        .unwrap();
    assert_eq!(get_updated_prayer_times_result.data, None);
    assert_eq!(
        get_updated_prayer_times_result.hash,
        "5a4e9c5d6b8a2f3e1c0b9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d".to_owned()
    );

    container.stop().await.expect("Container failed to stop");
}