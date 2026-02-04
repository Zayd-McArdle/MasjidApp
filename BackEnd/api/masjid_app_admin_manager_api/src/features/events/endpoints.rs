use crate::features::events::errors::{DeleteEventError, UpsertEventError};
use crate::features::events::repositories::EventsAdminRepository;
use crate::shared::jwt::Claims;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::features::events::endpoints::get_events_common;
use masjid_app_api_library::features::events::models::{Event, EventDTO};
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::extractors::file_handler::FileHandler;
use masjid_app_api_library::shared::extractors::request_validator::multipart::ValidatedMultipartRequest;
use masjid_app_api_library::shared::types::app_state::AppState;
use std::sync::Arc;
use validator::Validate;

pub async fn get_events(State(state): State<AppState<Arc<dyn EventsAdminRepository>>>) -> Response {
    get_events_common(State(state)).await
}

pub async fn upsert_events(
    State(state): State<AppState<Arc<dyn EventsAdminRepository>>>,
    file_uploader: FileHandler,
    claims: Claims,
    mut request: ValidatedMultipartRequest<EventDTO>,
) -> Response {
    if request.json.validate().is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    /* match file_uploader
        .save_file(&request.file_data, request.filename)
        .await
    {
        Ok(url) => {
            request.json.event_details.image_url = Some(url);
        }
        Err(err) => {
            match err {
                //If no file was uploaded, ignore
                UploadError::NoFileName => {}
                UploadError::EmptyFile | UploadError::InvalidFileName => {
                    return (StatusCode::UNPROCESSABLE_ENTITY, err.to_string()).into_response();
                }
                UploadError::UnsupportedFileType(file_type) => {
                    return (StatusCode::UNSUPPORTED_MEDIA_TYPE, file_type).into_response();
                }
                UploadError::SystemIOError => {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };
        }
    };*/

    let mut upsert_event_result = Err(UpsertEventError::UnableToUpsertEvent);

    if let Some(in_memory_repository) = state.repository_map.get(&DbType::InMemory) {
        upsert_event_result = in_memory_repository
            .upsert_event(Event::from(request.json.clone()))
            .await;
    }
    if upsert_event_result.is_err() {
        upsert_event_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .upsert_event(Event::from(request.json))
            .await;
    }
    match upsert_event_result {
        Ok(()) => StatusCode::OK.into_response(),
        Err(UpsertEventError::UnableToUpsertEvent) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_event(
    State(state): State<AppState<Arc<dyn EventsAdminRepository>>>,
    file_deleter: FileHandler,
    claims: Claims,
    Path(event_id): Path<i32>,
) -> Response {
    if event_id == 0 {
        return (StatusCode::BAD_REQUEST, "event ids cannot be 0").into_response();
    }

    let mut delete_event_result = Err(DeleteEventError::UnableToDeleteEvent);
    if let Some(in_memory_repository) = state.repository_map.get(&DbType::InMemory) {
        delete_event_result = in_memory_repository.delete_event_by_id(&event_id).await;
    }
    if delete_event_result.is_err() {
        delete_event_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .delete_event_by_id(&event_id)
            .await;
    }
    match delete_event_result {
        Ok(image_url) => {
            // TODO - implement file handling in a separate api
            /*if let Some(url) = image_url {
                let file_directory = url
                    .splitn(2, "//") // Split on double slash
                    .nth(1) // Take the part after protocol
                    .and_then(|s| s.splitn(2, '/').nth(1)); // Take the part after first single slash

                if let Some(file_directory) = file_directory {
                    if let Err(delete_file_err) = file_deleter.delete_file(file_directory).await {
                        return match delete_file_err {
                            DeleteError::FileNotFound => {
                                (StatusCode::NOT_FOUND, "file path in request uri not found")
                                    .into_response()
                            }
                            DeleteError::DirectoryNotFound => {
                                (StatusCode::NOT_FOUND, "endpoint in request not found")
                                    .into_response()
                            }
                            DeleteError::PathIsTraversal => {
                                (StatusCode::FORBIDDEN, "invalid url").into_response()
                            }
                            DeleteError::PermissionDenied => (
                                StatusCode::FORBIDDEN,
                                "requested file to be deleted cannot be done",
                            )
                                .into_response(),
                            DeleteError::DirectoryMistookForFile => (
                                StatusCode::UNPROCESSABLE_ENTITY,
                                "filename in request is a directory",
                            )
                                .into_response(),
                            DeleteError::UnableToDeleteFileDueToReadOnlyAccess => (
                                StatusCode::FORBIDDEN,
                                "requested file to be deleted cannot be done, as it is read only",
                            )
                                .into_response(),
                            DeleteError::UnableToDeleteFileDueToBeingInUse
                            | DeleteError::IOError(_) => {
                                StatusCode::INTERNAL_SERVER_ERROR.into_response()
                            }
                            DeleteError::EmptyPath => StatusCode::BAD_REQUEST.into_response(),
                        };
                    }
                } else {
                    return (StatusCode::BAD_REQUEST, format!("invalid path {url}"))
                        .into_response();
                }
            }*/
            StatusCode::OK.into_response()
        }
        Err(DeleteEventError::EventNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(DeleteEventError::UnableToDeleteEvent) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
mod test {
    use crate::features::events::endpoints::{delete_event, upsert_events};
    use crate::features::events::errors::{DeleteEventError, UpsertEventError};
    use crate::features::events::repositories::EventsAdminRepository;
    use crate::shared::jwt::Claims;
    use async_trait::async_trait;
    use axum::body::Bytes;
    use axum::extract::State;
    use axum::http::StatusCode;
    use masjid_app_api_library::features::events::errors::GetEventsError;
    use masjid_app_api_library::features::events::models::Event;
    use masjid_app_api_library::features::events::models::{
        EventDTO, EventDetails, EventRecurrence, EventStatus, EventType,
    };
    use masjid_app_api_library::features::events::repositories::EventsRepository;
    use masjid_app_api_library::shared::data_access::db_type::DbType;
    use masjid_app_api_library::shared::extractors::file_handler::FileHandler;
    use masjid_app_api_library::shared::extractors::request_validator::multipart::ValidatedMultipartRequest;
    use masjid_app_api_library::shared::types::age_range::AgeRange;
    use masjid_app_api_library::shared::types::app_state::AppState;
    use masjid_app_api_library::shared::types::contact_details::ContactDetails;
    use mockall::mock;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn get_valid_upsert_request(include_file: bool) -> ValidatedMultipartRequest<EventDTO> {
        let mut file_data = Bytes::default();
        let mut filename = String::default();
        if include_file {
            file_data = Bytes::from("test data");
            filename = "test_file_name.txt".to_owned();
        }
        ValidatedMultipartRequest {
            json: EventDTO {
                id: 0,
                title: "This is a title".to_owned(),
                description: None,
                date: Default::default(),
                event_details: EventDetails {
                    event_type: EventType::Talk,
                    event_recurrence: EventRecurrence::OneOff,
                    event_status: EventStatus::Confirmed,
                    age_range: Some(AgeRange {
                        minimum_age: 13,
                        maximum_age: 16,
                    }),
                    image_url: None,
                    contact_details: ContactDetails {
                        full_name: "John Smith".to_owned(),
                        title: None,
                        phone_number: "07787395729".to_owned(),
                        email: Some("johnsmith@test.com".to_owned()),
                    },
                },
            },
            file_data: Some(file_data),
            filename: Some(filename),
        }
    }
    mock!(
        pub EventsAdminRepository {}

        #[async_trait]
        impl EventsRepository for EventsAdminRepository {
            async fn get_events(&self) -> Result<Vec<EventDTO>, GetEventsError>;
        }
        #[async_trait]
        impl EventsAdminRepository for EventsAdminRepository {
            async fn upsert_event(&self, event: Event) -> Result<(), UpsertEventError>;
            async fn delete_event_by_id(&self, event_id: &i32) -> Result<Option<String>, DeleteEventError>;
        }
    );
    #[tokio::test]
    async fn test_upsert_event() {
        struct TestCase {
            request: ValidatedMultipartRequest<EventDTO>,
            file_uploader: FileHandler,
            expected_in_memory_db_response: Option<Result<(), UpsertEventError>>,
            expected_db_response: Option<Result<(), UpsertEventError>>,
            expected_status: StatusCode,
        }
        let test_cases = [
            //Given the request json is invalid, I should get a bad request
            TestCase {
                request: ValidatedMultipartRequest {
                    json: EventDTO {
                        id: 0,
                        title: "".to_owned(),
                        description: None,
                        date: Default::default(),
                        event_details: EventDetails {
                            event_type: EventType::Talk,
                            event_recurrence: EventRecurrence::OneOff,
                            event_status: EventStatus::Cancelled,
                            age_range: None,
                            image_url: None,
                            contact_details: ContactDetails {
                                full_name: "".to_owned(),
                                title: None,
                                phone_number: "".to_owned(),
                                email: None,
                            },
                        },
                    },
                    file_data: None,
                    filename: None,
                },
                file_uploader: FileHandler::default(),
                expected_in_memory_db_response: None,
                expected_db_response: None,
                expected_status: StatusCode::BAD_REQUEST,
            },
            // Given the json is valid, but event upsertion fails in database, I should get an internal server error
            TestCase {
                request: get_valid_upsert_request(false),
                file_uploader: FileHandler::default(),
                expected_in_memory_db_response: Some(Err(UpsertEventError::UnableToUpsertEvent)),
                expected_db_response: Some(Err(UpsertEventError::UnableToUpsertEvent)),
                expected_status: StatusCode::INTERNAL_SERVER_ERROR,
            },
            // Given the json is valid and upsertion succeeds, I should get an ok response
            TestCase {
                request: get_valid_upsert_request(false),
                file_uploader: FileHandler::default(),
                expected_in_memory_db_response: Some(Ok(())),
                expected_db_response: Some(Ok(())),
                expected_status: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            let mut mock_in_memory_repository = MockEventsAdminRepository::new();
            let mut mock_repository = MockEventsAdminRepository::new();
            if let Some(expected_in_memory_db_response) = test_case.expected_in_memory_db_response {
                mock_in_memory_repository
                    .expect_upsert_event()
                    .returning(move |data| expected_in_memory_db_response);
            }
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_repository
                    .expect_upsert_event()
                    .returning(move |data| expected_db_response);
            }
            let arc_repository: Arc<dyn EventsAdminRepository> = Arc::new(mock_repository);
            let arc_in_memory_repository: Arc<dyn EventsAdminRepository> =
                Arc::new(mock_in_memory_repository);
            let app_state: AppState<Arc<dyn EventsAdminRepository>> = AppState {
                repository_map: HashMap::from([
                    (DbType::MySql, arc_repository),
                    (DbType::InMemory, arc_in_memory_repository),
                ]),
            };
            let actual_response = upsert_events(
                State(app_state),
                test_case.file_uploader,
                Claims::default(),
                test_case.request,
            )
            .await;
            assert_eq!(test_case.expected_status, actual_response.status());
        }
    }

    #[tokio::test]
    async fn test_delete_event() {
        struct TestCase {
            description: &'static str,
            delete_event_request_id: i32,
            file_deleter: FileHandler,
            expected_in_memory_db_response: Option<Result<Option<String>, DeleteEventError>>,
            expected_db_response: Option<Result<Option<String>, DeleteEventError>>,
            expected_status: StatusCode,
        }
        let test_cases = [
            TestCase {
                description: "When I use an invalid event ID, I should get a bad request",
                delete_event_request_id: 0,
                file_deleter: FileHandler::default(),
                expected_in_memory_db_response: None,
                expected_db_response: None,
                expected_status: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "When I delete an event using a non-existent ID, I should get a not found",
                delete_event_request_id: 1,
                file_deleter: FileHandler::default(),
                expected_in_memory_db_response: None,
                expected_db_response: Some(Err(DeleteEventError::EventNotFound)),
                expected_status: StatusCode::NOT_FOUND,
            },
            TestCase {
                description: "When deleting an event fails, I should get an internal server error",
                delete_event_request_id: 2,
                file_deleter: FileHandler::default(),
                expected_in_memory_db_response: None,
                expected_db_response: Some(Err(DeleteEventError::UnableToDeleteEvent)),
                expected_status: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "When deleting an event succeeds, I should get an ok response",
                delete_event_request_id: 2,
                file_deleter: FileHandler::default(),
                expected_in_memory_db_response: None,
                expected_db_response: Some(Ok(None)),
                expected_status: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_in_memory_repository = MockEventsAdminRepository::new();
            let mut mock_repository = MockEventsAdminRepository::new();
            if let Some(expected_in_memory_db_response) = test_case.expected_in_memory_db_response {
                mock_in_memory_repository
                    .expect_delete_event_by_id()
                    .return_once(move |_| expected_in_memory_db_response);
            }
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_repository
                    .expect_delete_event_by_id()
                    .return_once(move |_| expected_db_response);
            }

            let arc_repository: Arc<dyn EventsAdminRepository> = Arc::new(mock_repository);
            let arc_in_memory_repository: Arc<dyn EventsAdminRepository> =
                Arc::new(mock_in_memory_repository);
            let app_state: AppState<Arc<dyn EventsAdminRepository>> = AppState {
                repository_map: HashMap::from([
                    (DbType::MySql, arc_repository),
                    //(DbType::InMemory, arc_in_memory_repository),
                ]),
            };
            let actual_response = delete_event(
                State(app_state),
                test_case.file_deleter,
                Claims::default(),
                axum::extract::Path(test_case.delete_event_request_id),
            )
            .await;
            assert_eq!(test_case.expected_status, actual_response.status());
        }
    }
}
