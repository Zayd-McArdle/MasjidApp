use crate::features::events::repositories::errors::delete_event_error::DeleteEventError;
use crate::features::events::services::errors::event_deletion_error::EventDeletionError;
use crate::features::events::services::event_deletion_service::EventDeletionService;
use crate::shared::jwt::Claims;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::shared::extractors::file_handler::FileHandler;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

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
mod tests {
    use super::*;
    use crate::features::events::services::event_deletion_service::MockEventDeletionService;
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
