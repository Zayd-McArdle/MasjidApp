use crate::features::events::errors::GetEventsRepositoryError;
use crate::features::events::services::errors::get_events_service_error::GetEventsServiceError;
use crate::features::events::services::event_retrieval_service::EventRetrievalService;
use crate::shared::types::app_state::ServiceAppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::sync::Arc;

#[inline]
pub async fn get_events_common<R: EventRetrievalService + ?Sized>(
    State(state): State<ServiceAppState<Arc<R>>>,
) -> Response {
    match state.service.get_events().await {
        Ok(events) => (StatusCode::OK, Json(events)).into_response(),

        Err(GetEventsServiceError::UnableToGetEventsFromRepository(
            GetEventsRepositoryError::EventsNotFound,
        )) => StatusCode::NOT_FOUND.into_response(),
        Err(GetEventsServiceError::UnableToGetEventsFromRepository(
            GetEventsRepositoryError::UnableToGetEvents,
        )) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::features::events::models::{
        EventDTO, EventDetails, EventRecurrence, EventStatus, EventType,
    };
    use crate::features::events::services::event_retrieval_service::MockEventRetrievalService;
    use crate::shared::types::age_range::AgeRange;
    use crate::shared::types::contact_details::ContactDetails;

    #[tokio::test]
    async fn test_get_events_common() {
        let events = vec![EventDTO {
            id: 0,
            title: "This is a title".to_owned(),
            description: Some("This is a description".to_owned()),
            date: Default::default(),
            event_details: EventDetails {
                event_type: EventType::Talk,
                event_recurrence: EventRecurrence::OneOff,
                event_status: EventStatus::Confirmed,
                age_range: Some(AgeRange {
                    minimum_age: 16,
                    maximum_age: 18,
                }),
                image_url: None,
                contact_details: ContactDetails {
                    full_name: "John Smith".to_owned(),
                    title: None,
                    phone_number: "07127665431".to_owned(),
                    email: Some("johns.smith@masjidapp.com".to_owned()),
                },
            },
        }];
        struct TestCase {
            description: &'static str,
            expected_service_response: Result<Vec<EventDTO>, GetEventsServiceError>,
            expected_response_code: StatusCode,
        }
        let test_cases = vec![
            TestCase {
                description: "When retrieval of events fails",
                expected_service_response: Err(
                    GetEventsServiceError::UnableToGetEventsFromRepository(
                        GetEventsRepositoryError::UnableToGetEvents,
                    ),
                ),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "When no events found",
                expected_service_response: Err(
                    GetEventsServiceError::UnableToGetEventsFromRepository(
                        GetEventsRepositoryError::EventsNotFound,
                    ),
                ),
                expected_response_code: StatusCode::NOT_FOUND,
            },
            TestCase {
                description: "When events found",
                expected_service_response: Ok(events),
                expected_response_code: StatusCode::OK,
            },
        ];

        for case in test_cases {
            eprintln!("{}", case.description);
            let mut mock_service = MockEventRetrievalService::new();

            mock_service
                .expect_get_events()
                .return_once(move || case.expected_service_response);

            let app_state = ServiceAppState::<Arc<dyn EventRetrievalService>> {
                service: Arc::new(mock_service),
            };

            let actual_response = get_events_common(State(app_state)).await;
            assert_eq!(actual_response.status(), case.expected_response_code)
        }
    }
}
