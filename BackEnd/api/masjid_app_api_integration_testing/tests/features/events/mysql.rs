use masjid_app_admin_manager_api::features::events::errors::DeleteEventError;
use masjid_app_admin_manager_api::features::events::repositories::new_events_admin_repository;
use masjid_app_api_library::features::events::errors::GetEventsError;
use masjid_app_api_library::features::events::models::{Event, EventDTO, EventRecurrence, EventStatus, EventType};
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_public_api::features::events::repositories::new_events_public_repository;
use crate::common::data_access_layer;
use crate::common::data_access_layer::DatabaseCredentials;
use crate::common::logging::setup_logging;

#[tokio::test]
async fn test_events() {
    setup_logging();
    let container = data_access_layer::mysql::setup_container(DatabaseCredentials {
        username: "eventsadmin".to_string(),
        password: "changeme".to_string(),
        environment_variable: "EVENTS_CONNECTION".to_string(),
    }).await;

    let public_repository = new_events_public_repository(DbType::MySql).await;
    let admin_repository = new_events_admin_repository(DbType::MySql).await;



    // Given no events exist, I should receive an error when deleting an event
    let delete_event_result = admin_repository.delete_event_by_id(&1).await.unwrap_err();
    assert_eq!(delete_event_result, DeleteEventError::EventNotFound);

    // When I try to retrieve events from an empty database, I should get an error
    let get_events_result = public_repository.get_events().await.unwrap_err();
    assert_eq!(get_events_result, GetEventsError::EventsNotFound);
    let mut event = Event{
        id: 0,
        title: "This is my event".to_string(),
        description: Some("This is my description".to_owned()),
        date: "2023-12-25T15:30:00Z".parse().unwrap(),
        r#type: EventType::Talk.to_string(),
        recurrence: EventRecurrence::OneOff.to_string(),
        status: EventStatus::Confirmed.to_string(),
        minimum_age: Some(14),
        maximum_age: Some(16),
        image_url: None,
        full_name: "John Smith".to_string(),
        phone_number: "07123456789".to_string(),
        email: None,
    };

    // When I insert a new event, I should get no error
    let insert_event_result = admin_repository.upsert_event(event.clone()).await;
    assert!(insert_event_result.is_ok());

    // When I retrieve events, I should get the event that I inserted
    let get_events_result = public_repository.get_events().await.unwrap();
    event.id = 1;
    assert_eq!(get_events_result, vec![EventDTO::from(event)]);
    // When I update my event, I should get no error
    let event = Event{
        id: 1,
        title: "This is my updated event".to_string(),
        description: Some("This is my updated description".to_owned()),
        date: "2023-12-25T15:30:00Z".parse().unwrap(),
        r#type: EventType::Social.to_string(),
        recurrence: EventRecurrence::OneOff.to_string(),
        status: EventStatus::Confirmed.to_string(),
        minimum_age: Some(19),
        maximum_age: Some(25),
        image_url: None,
        full_name: "John Smith".to_string(),
        phone_number: "07123456789".to_string(),
        email: None,
    };

    let update_result = admin_repository.upsert_event(event.clone()).await;
    assert!(update_result.is_ok());

    // When I retrieve events, I should get my updated event
    let get_events_result = public_repository.get_events().await.unwrap();
    assert_eq!(get_events_result, vec![EventDTO::from(event)]);

    // When I delete an event, I should get no error
    let delete_event_result = admin_repository.delete_event_by_id(&1).await;
    assert!(delete_event_result.is_ok());

    // When trying to retrieve events, I should get an error
    let get_events_result = public_repository.get_events().await.unwrap_err();
    assert_eq!(get_events_result, GetEventsError::EventsNotFound);
    container.stop().await.unwrap();
}