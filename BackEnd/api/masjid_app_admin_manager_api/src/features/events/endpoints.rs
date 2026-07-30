use crate::features::events::errors::{DeleteEventError, InsertEventError, UpdateEventError, UpsertEventError};
use crate::features::events::repositories::EventsAdminRepository;
use crate::features::events::services::errors::event_deletion_error::EventDeletionError;
use crate::features::events::services::errors::event_publishing_error::EventPublishingError;
use crate::features::events::services::event_deletion_service::EventDeletionService;
use crate::features::events::services::event_publishing_service::EventPublishingService;
use crate::shared::jwt::Claims;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::features::events::endpoints::get_events_common;
use masjid_app_api_library::features::events::models::EventDTO;
use masjid_app_api_library::features::events::services::event_retrieval_service::EventRetrievalService;
use masjid_app_api_library::shared::extractors::file_handler::FileHandler;
use masjid_app_api_library::shared::extractors::request_validator::multipart::ValidatedMultipartRequest;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

pub async fn get_events(
    State(state): State<ServiceAppState<Arc<dyn EventRetrievalService>>>,
) -> Response {
    get_events_common(State(state)).await
}

pub async fn upsert_events(
    State(state): State<ServiceAppState<Arc<dyn EventPublishingService>>>,
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

    match state.service.publish_event(request.json).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(EventPublishingError::RepositoryError(UpsertEventError::InsertError(
            InsertEventError::EventAlreadyExists,
        ))) => StatusCode::CONFLICT.into_response(),

        Err(EventPublishingError::RepositoryError(UpsertEventError::UpdateError(
            UpdateEventError::EventNotFound,
        ))) => StatusCode::NOT_FOUND.into_response(),
        Err(EventPublishingError::UnableToSaveImage)
        | Err(EventPublishingError::RepositoryError(UpsertEventError::InsertError(
            InsertEventError::UnableToInsertEvent,
        )))
        | Err(EventPublishingError::RepositoryError(UpsertEventError::UpdateError(
            UpdateEventError::UnableToUpdateEvent,
        )))
        | Err(EventPublishingError::RepositoryError(UpsertEventError::UnableToUpsertEvent)) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_event(
    State(state): State<ServiceAppState<Arc<dyn EventDeletionService>>>,
    file_deleter: FileHandler,
    claims: Claims,
    Path(event_id): Path<i32>,
) -> Response {
    if event_id == 0 {
        return (StatusCode::BAD_REQUEST, "event ids cannot be 0").into_response();
    }

    match state.service.delete_event(event_id).await {
        Ok(()) => {
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
        Err(EventDeletionError::RepositoryError(DeleteEventError::EventNotFound)) => {
            StatusCode::NOT_FOUND.into_response()
        }
        Err(EventDeletionError::RepositoryError(DeleteEventError::UnableToDeleteEvent))
        | Err(EventDeletionError::UnableToDeleteImagesRelatedToEvent) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::features::events::services::event_deletion_service::MockEventDeletionService;
    use crate::features::events::services::event_publishing_service::MockEventPublishingService;
    use axum::body::Bytes;
    use masjid_app_api_library::features::events::models::{
        EventDetails, EventRecurrence, EventStatus, EventType,
    };
    use masjid_app_api_library::shared::types::age_range::AgeRange;
    use masjid_app_api_library::shared::types::contact_details::ContactDetails;

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

    #[tokio::test]
    async fn test_upsert_event() {
        struct TestCase {
            description: &'static str,
            request: ValidatedMultipartRequest<EventDTO>,
            file_uploader: FileHandler,
            expected_service_response: Option<Result<(), EventPublishingError>>,
            expected_status: StatusCode,
        }
        let test_cases = [
            TestCase {
                description: "Given the request json is invalid, I should get a bad request",
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
                expected_service_response: None,
                expected_status: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "Given the json is valid, but event publishing fails, I should get an internal server error",
                request: get_valid_upsert_request(false),
                file_uploader: FileHandler::default(),
                expected_service_response: Some(Err(EventPublishingError::RepositoryError(
                    UpsertEventError::UnableToUpsertEvent,
                ))),
                expected_status: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "Given the json is valid and upsertion succeeds, I should get an ok response",
                request: get_valid_upsert_request(false),
                file_uploader: FileHandler::default(),
                expected_service_response: Some(Ok(())),
                expected_status: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockEventPublishingService::new();
            if let Some(mock_response) = test_case.expected_service_response {
                mock_service
                    .expect_publish_event()
                    .return_once(move |_| mock_response);
            }
            let app_state = ServiceAppState::<Arc<dyn EventPublishingService>> {
                service: Arc::new(mock_service),
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
            expected_service_response: Option<Result<(), EventDeletionError>>,
            expected_status: StatusCode,
        }
        let test_cases = [
            TestCase {
                description: "When I use an invalid event ID, I should get a bad request",
                delete_event_request_id: 0,
                file_deleter: FileHandler::default(),
                expected_service_response: None,
                expected_status: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "When I delete an event using a non-existent ID, I should get a not found",
                delete_event_request_id: 1,
                file_deleter: FileHandler::default(),
                expected_service_response: Some(Err(EventDeletionError::RepositoryError(
                    DeleteEventError::EventNotFound,
                ))),
                expected_status: StatusCode::NOT_FOUND,
            },
            TestCase {
                description: "When deleting an event fails, I should get an internal server error",
                delete_event_request_id: 2,
                file_deleter: FileHandler::default(),
                expected_service_response: None,
                expected_status: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "When deleting an event succeeds, I should get an ok response",
                delete_event_request_id: 2,
                file_deleter: FileHandler::default(),
                expected_service_response: None,
                expected_status: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockEventDeletionService::new();
            if let Some(mock_response) = test_case.expected_service_response {
                mock_service
                    .expect_delete_event()
                    .return_once(move |_| mock_response);
            }

            let app_state = ServiceAppState::<Arc<dyn EventDeletionService>> {
                service: Arc::new(mock_service),
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
