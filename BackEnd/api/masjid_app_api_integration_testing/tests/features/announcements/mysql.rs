/*#[tokio::test]
async fn test_announcements() {
    setup_logging();
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
*/