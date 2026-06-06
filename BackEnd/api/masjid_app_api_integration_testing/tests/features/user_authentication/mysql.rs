use crate::common::data_access_layer::{mysql, DatabaseCredentials};
use crate::common::logging::setup_logging;
use masjid_app_admin_manager_api::features::user_authentication::errors::{
    GetUserError, UpdateUserPasswordError,
};
use masjid_app_admin_manager_api::features::user_authentication::models::UserAccountDTO;
use masjid_app_admin_manager_api::features::user_authentication::repositories::new_user_repository;
#[tokio::test]
async fn test_user_authentication() {
    setup_logging();
    let main_database_container = mysql::setup_container(DatabaseCredentials {
        username: "authenticationuser".to_string(),
        password: "BL6FxKu!237GvPS9".to_string(),
        environment_variable: "AUTHENTICATION_CONNECTION".to_string(),
    })
    .await;

    //Given no user exists, I should get an error when attempting to log in
    let repository = new_user_repository().await;
    let login_result = repository
        .get_user_by_credentials("JohnSmith", "password")
        .await
        .unwrap_err();
    assert_eq!(login_result, GetUserError::NotFound);

    //Given no user exists, I should get an error when trying to reset the user password
    let reset_password_result = repository
        .update_user_password("JohnSmith", "new_password")
        .await
        .unwrap_err();
    assert_eq!(
        reset_password_result,
        UpdateUserPasswordError::UserDoesNotExist
    );

    //Given no user exists, I should successfully register one with no error
    let new_user = UserAccountDTO {
        full_name: "John Smith".to_string(),
        email: "JohnSmith@masjidapp.com".to_string(),
        role: "Admin".to_string(),
        username: "JohnSmith".to_string(),
        password: "password".to_string(),
    };
    let register_result = repository.insert_new_user(new_user).await;
    assert!(register_result.is_ok());

    //Given a new user has been created, I should be able to successfully log in
    let login_result = repository
        .get_user_by_credentials("JohnSmith", "password")
        .await
        .unwrap();
    assert_eq!(login_result, "Admin".to_string());

    //Given a user exists, I should be able to reset their password
    let reset_password_result = repository
        .update_user_password("JohnSmith", "new_password")
        .await;
    assert!(reset_password_result.is_ok());

    //Given the user reset their password, they should be able to login using it
    let login_result = repository
        .get_user_by_credentials("JohnSmith", "new_password")
        .await
        .unwrap();
    assert_eq!(login_result, "Admin".to_string());

    main_database_container
        .stop()
        .await
        .expect("Container failed to stop");
}
