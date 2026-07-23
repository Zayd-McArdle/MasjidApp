use crate::common::data_access_layer::{mysql, DatabaseCredentials};
use crate::common::logging::setup_logging;
use masjid_app_admin_manager_api::features::user_authentication::models::UserAccountDTO;
use masjid_app_admin_manager_api::features::user_authentication::repositories::new_user_repository;
use masjid_app_admin_manager_api::features::user_authentication::services::login_service::new_login_service;
use masjid_app_admin_manager_api::features::user_authentication::services::reset_password_service::new_reset_password_service;
use masjid_app_admin_manager_api::features::user_authentication::services::user_registration_service::new_user_registration_service;
use masjid_app_api_library::shared::services::hashing::providers::HashingProvider;
use masjid_app_api_library::shared::services::hashing::r#trait::new_hashing_service;

#[tokio::test]
async fn test_user_authentication_service() {
    setup_logging();
    let main_database_container = mysql::setup_container(DatabaseCredentials {
        username: "authenticationuser".to_owned(),
        password: "BL6FxKu!237GvPS9".to_owned(),
        environment_variable: "AUTHENTICATION_CONNECTION".to_string(),
    })
    .await;
    let hashing_service = new_hashing_service(HashingProvider::Bcrypt);

    let user_registration_service =
        new_user_registration_service(hashing_service.clone(), new_user_repository().await);
    let registration_result = user_registration_service
        .register_user(UserAccountDTO {
            full_name: "zayd mcardle".to_string(),
            email: "zaydmcardle@masjidapp.com".to_string(),
            role: "admin".to_string(),
            username: "zayd".to_string(),
            password: "1234".to_string(),
        })
        .await;
    assert!(registration_result.is_ok());

    let login_service = new_login_service(hashing_service.clone(), new_user_repository().await);
    let mut login_result = login_service.login("zayd", "1234").await;
    assert!(login_result.is_ok());

    let password_reset_service =
        new_reset_password_service(hashing_service, new_user_repository().await);
    let password_reset_result = password_reset_service
        .reset_password("zayd", "1234321")
        .await;
    assert!(password_reset_result.is_ok());

    login_result = login_service.login("zayd", "1234321").await;
    assert!(login_result.is_ok());

    main_database_container
        .stop()
        .await
        .expect("Container failed to stop");
}
