use crate::common::dummy_data::add_user_to_main_database;
use crate::common::setup::{setup_main_database, DatabaseCredentials};
use chrono::DateTime;
use masjid_app_api::features::announcements::{
    new_announcement_repository, AnnouncementDTO, EditAnnouncementError, GetAnnouncementsError,
};
use masjid_app_api::features::prayer_times::{new_prayer_times_repository, GetPrayerTimesError};
use masjid_app_api::features::user_authentication;
use masjid_app_api::features::user_authentication::{
    LoginError, ResetPasswordError, UserAccountDTO,
};
use masjid_app_api::shared::app_state::DbType;

#[tokio::test]
async fn test_user_authentication() {
    let main_database_container = setup_main_database(DatabaseCredentials {
        username: "authenticationuser".to_string(),
        password: "BL6FxKu!237GvPS9".to_string(),
        environment_variable: "AUTHENTICATION_CONNECTION".to_string(),
    })
    .await;

    //Given no user exists, I should get an error when attempting to log in
    let repository = user_authentication::new_user_repository(DbType::MySql).await;
    let login_result = repository.login("JohnSmith", "password").await.unwrap_err();
    assert_eq!(login_result, LoginError::InvalidCredentials);

    //Given no user exists, I should get an error when trying to reset the user password
    let reset_password_result = repository
        .reset_user_password("JohnSmith", "new_password")
        .await
        .unwrap_err();
    assert_eq!(reset_password_result, ResetPasswordError::UserDoesNotExist);

    //Given no user exists, I should successfully register one with no error
    let new_user = UserAccountDTO {
        full_name: "John Smith".to_string(),
        email: "JohnSmith@masjidapp.com".to_string(),
        role: "Admin".to_string(),
        username: "JohnSmith".to_string(),
        password: "password".to_string(),
    };
    let register_result = repository.register_user(new_user).await;
    assert!(register_result.is_ok());

    //Given a new user has been created, I should be able to successfully log in
    let login_result = repository.login("JohnSmith", "password").await.unwrap();
    assert_eq!(login_result, "Admin".to_string());

    //Given a user exists, I should be able to reset their password
    let reset_password_result = repository
        .reset_user_password("JohnSmith", "new_password")
        .await;
    assert!(reset_password_result.is_ok());

    //Given the user reset their password, they should be able to login using it
    let login_result = repository.login("JohnSmith", "new_password").await.unwrap();
    assert_eq!(login_result, "Admin".to_string());

    main_database_container
        .stop()
        .await
        .expect("Container failed to stop");
}

#[tokio::test]
async fn test_announcements() {
    let main_database_container = setup_main_database(DatabaseCredentials {
        username: "announcementsuser".to_string(),
        password: "LzwvN6bU4y3EqmAYBMJFrn".to_string(),
        environment_variable: "ANNOUNCEMENT_CONNECTION".to_string(),
    })
    .await;
    unsafe {
        let port = main_database_container
            .get_host_port_ipv4(3306)
            .await
            .unwrap();
        let connection_string = format!(
            "mysql://authenticationuser:BL6FxKu!237GvPS9@127.0.0.1:{port}/masjidappdatabase"
        );
        std::env::set_var("AUTHENTICATION_CONNECTION", connection_string);
    }
    DatabaseCredentials {
        username: "authenticationuser".to_string(),
        password: "BL6FxKu!237GvPS9".to_string(),
        environment_variable: "AUTHENTICATION_CONNECTION".to_string(),
    };

    add_user_to_main_database(UserAccountDTO {
        full_name: "John Smith".to_string(),
        email: "JohnSmith@masjidapp.com".to_string(),
        role: "Admin".to_string(),
        username: "JohnSmith".to_string(),
        password: "password".to_string(),
    })
    .await;
    let repository = new_announcement_repository(DbType::MySql).await;
    let retrieved_announcement = AnnouncementDTO {
        id: 1,
        title: "This is my announcement".to_string(),
        description: Some("This is the description to my announcement".to_string()),
        last_updated: DateTime::default(),
        image: None,
        author: "John Smith".to_string(),
    };

    let mut announcement_to_post = retrieved_announcement.clone();
    announcement_to_post.author = "JohnSmith".to_owned();

    let retrieved_edited_announcement = AnnouncementDTO {
        id: 1,
        title: "This is my edited announcement".to_string(),
        description: Some("This is the description to my edited announcement".to_string()),
        last_updated: DateTime::default(),
        image: None,
        author: "John Smith".to_string(),
    };

    let mut announcement_to_edit = retrieved_edited_announcement.clone();
    announcement_to_edit.author = "JohnSmith".to_owned();

    //Given no announcements exist, I should receive an error
    let get_announcements_result = repository.get_announcements().await.unwrap_err();
    assert_eq!(
        get_announcements_result,
        GetAnnouncementsError::AnnouncementsNotFound
    );

    //When editing a non-existent announcement, I should receive an error
    let edit_announcement_result = repository
        .edit_announcement(announcement_to_edit.clone())
        .await
        .unwrap_err();
    assert_eq!(
        edit_announcement_result,
        EditAnnouncementError::AnnouncementDoesNotExist
    );

    //When posting a new announcement, I should get no error
    let post_announcement_result = repository.post_announcement(announcement_to_post).await;
    assert!(post_announcement_result.is_ok());

    //When retrieving announcements, I should see my newly posted announcement
    let get_announcements_result = repository.get_announcements().await.unwrap();
    assert_eq!(get_announcements_result.len(), 1);
    assert_eq!(
        get_announcements_result[0].title,
        retrieved_announcement.title
    );
    assert_eq!(
        get_announcements_result[0].description,
        retrieved_announcement.description
    );
    assert_eq!(
        get_announcements_result[0].image,
        retrieved_announcement.image
    );
    assert_eq!(
        get_announcements_result[0].author,
        retrieved_announcement.author
    );

    //When editing an existing announcement, I should get no error
    let edit_announcement_result = repository.edit_announcement(announcement_to_edit).await;
    assert!(edit_announcement_result.is_ok());

    //When retrieving announcements, I should see my edited announcement
    let get_announcements_result = repository.get_announcements().await.unwrap();
    assert_eq!(get_announcements_result.len(), 1);
    assert_eq!(
        get_announcements_result[0].title,
        retrieved_edited_announcement.title
    );
    assert_eq!(
        get_announcements_result[0].description,
        retrieved_edited_announcement.description
    );
    assert_eq!(
        get_announcements_result[0].image,
        retrieved_edited_announcement.image
    );
    assert_eq!(
        get_announcements_result[0].author,
        retrieved_edited_announcement.author
    );

    main_database_container
        .stop()
        .await
        .expect("Container failed to stop");
}

#[tokio::test]
async fn test_prayer_times() {
    let container = setup_main_database(DatabaseCredentials {
        username: "prayertimesuser".to_string(),
        password: "HR0o8NRkwvuMaIBh7yaf".to_string(),
        environment_variable: "PRAYER_TIMES_CONNECTION".to_string(),
    })
    .await;

    let repository = new_prayer_times_repository(DbType::MySql).await;

    //Given no prayer times exist, I should receive an error
    let get_prayer_times_result = repository.get_prayer_times().await.unwrap_err();
    assert_eq!(
        get_prayer_times_result,
        GetPrayerTimesError::PrayerTimesNotFound
    );

    //When I insert some prayer times, I should receive no error
    let update_prayer_times_result = repository
        .update_prayer_times(&vec![0x01, 0x02, 0x03, 0x04])
        .await;
    assert!(update_prayer_times_result.is_ok());

    //When I retrieve prayer times, I should receive no error
    let get_prayer_times_result = repository.get_prayer_times().await.unwrap();
    assert_eq!(get_prayer_times_result, vec![0x01, 0x02, 0x03, 0x04]);
    container.stop().await.expect("Container failed to stop");
}
