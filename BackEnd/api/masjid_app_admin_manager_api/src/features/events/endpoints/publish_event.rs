use crate::features::events::repositories::errors::insert_event_error::InsertEventError;
use crate::features::events::repositories::errors::update_event_error::UpdateEventError;
use crate::features::events::repositories::errors::upsert_event_error::UpsertEventError;
use crate::features::events::services::errors::event_publishing_error::EventPublishingError;
use crate::features::events::services::event_publishing_service::EventPublishingService;
use crate::shared::jwt::Claims;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::features::events::models::event_dto::EventDTO;
use masjid_app_api_library::shared::extractors::file_handler::FileHandler;
use masjid_app_api_library::shared::extractors::request_validator::multipart::ValidatedMultipartRequest;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;
use validator::Validate;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::events::services::event_publishing_service::MockEventPublishingService;
    use axum::body::Bytes;
    use masjid_app_api_library::features::events::models::event_details::EventDetails;
    use masjid_app_api_library::features::events::models::event_recurrence::EventRecurrence;
    use masjid_app_api_library::features::events::models::event_status::EventStatus;
    use masjid_app_api_library::features::events::models::event_type::EventType;
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
}
