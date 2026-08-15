use crate::common::data_access_layer::{DatabaseCredentials, mysql};
use crate::common::logging::setup_logging;
use masjid_app_admin_manager_api::features::user_authentication::errors::get_user_error::GetUserError;
use masjid_app_admin_manager_api::features::user_authentication::errors::update_user_password_error::UpdateUserPasswordError;
use masjid_app_admin_manager_api::features::user_authentication::models::login_dto::LoginDTO;
use masjid_app_admin_manager_api::features::user_authentication::models::user_account_dto::UserAccountDTO;
use masjid_app_admin_manager_api::features::user_authentication::repositories::new_user_repository;
#[tokio::test]
async fn test_user_authentication() {
    setup_logging();
    let main_database_container = mysql::setup_container(DatabaseCredentials {
        username: "authenticationuser".to_owned(),
        password: "BL6FxKu!237GvPS9".to_owned(),
        environment_variable: "AUTHENTICATION_CONNECTION".to_string(),
    })
    .await;

    //Given no user exists, I should get an error when attempting to log in
    let repository = new_user_repository().await;
    let login_result = repository
        .get_user_by_credentials("JohnSmith", "password")
        .await
        .unwrap_err();
    assert!(matches!(login_result, GetUserError::NotFound));

    //Given no user exists, I should get an error when trying to reset the user password
    let reset_password_result = repository
        .update_user_password("JohnSmith", "new_password")
        .await
        .unwrap_err();
    assert!(matches!(
        reset_password_result,
        UpdateUserPasswordError::UserDoesNotExist
    ));

    //Given no user exists, I should successfully register one with no error
    let new_user = UserAccountDTO {
        full_name: "John Smith".to_owned(),
        email: "JohnSmith@masjidapp.com".to_owned(),
        role: "Admin".to_owned(),
        username: "JohnSmith".to_owned(),
        password: "password".to_owned(),
    };
    let register_result = repository.insert_new_user(new_user).await;
    assert!(register_result.is_ok());

    //Given a new user has been created, I should be able to successfully log in
    let mut actual_login_result = repository
        .get_user_by_credentials("JohnSmith", "password")
        .await;
    let mut expected_login_result: Result<LoginDTO, GetUserError> = Ok(LoginDTO {
        username: "JohnSmith".to_owned(),
        password: "password".to_owned(),
        role: "Admin".to_owned(),
    });
    assert!(matches!(expected_login_result, actual_login_result));

    //Given a user exists, I should be able to reset their password
    let reset_password_result = repository
        .update_user_password("JohnSmith", "new_password")
        .await;
    assert!(reset_password_result.is_ok());

    //Given the user reset their password, they should be able to login using it
    actual_login_result = repository
        .get_user_by_credentials("JohnSmith", "new_password")
        .await;
    expected_login_result = Ok(LoginDTO {
        username: "JohnSmith".to_owned(),
        password: "new_password".to_owned(),
        role: "Admin".to_owned(),
    });
    assert!(matches!(expected_login_result, actual_login_result));

    main_database_container
        .stop()
        .await
        .expect("Container failed to stop");
}
