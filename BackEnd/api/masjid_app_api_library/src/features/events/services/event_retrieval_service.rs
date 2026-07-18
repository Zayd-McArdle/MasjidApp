use crate::features::events::models::EventDTO;
use crate::features::events::repositories::EventsRepository;
use crate::features::events::services::errors::get_events_service_error::GetEventsServiceError;
use crate::features::events::services::event_service_impl::EventServiceImpl;
use crate::new_event_service;
use crate::shared::common_service_impl::CommonServiceImpl;
use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait EventRetrievalService: Send + Sync {
    async fn get_events(&self) -> Result<Vec<EventDTO>, GetEventsServiceError>;
}

new_event_service!(
    new_event_retrieval_service,
    EventRetrievalService,
    EventsRepository
);

#[async_trait]
impl EventRetrievalService for EventServiceImpl<dyn EventsRepository> {
    async fn get_events(&self) -> Result<Vec<EventDTO>, GetEventsServiceError> {
        if let Ok(events) = self.common.in_memory_repository.get_events().await {
            Ok(events)
        } else {
            self.common
                .repository
                .get_events()
                .await
                .map_err(GetEventsServiceError::from)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::events::errors::GetEventsRepositoryError;
    use crate::features::events::models::{EventDetails, EventRecurrence, EventStatus, EventType};
    use crate::features::events::repositories::MockEventsRepository;
    use crate::shared::types::contact_details::ContactDetails;
    use chrono::DateTime;

    #[tokio::test]
    async fn test_event_retrieval_service_get_events() {
        struct TestCase {
            description: &'static str,
            expected_in_memory_db_response: Result<Vec<EventDTO>, GetEventsRepositoryError>,
            expected_db_response: Option<Result<Vec<EventDTO>, GetEventsRepositoryError>>,
            expected_result: Result<Vec<EventDTO>, GetEventsServiceError>,
        }
        let events = vec![EventDTO {
            id: 0,
            title: "some event".to_owned(),
            description: Some("some description".to_owned()),
            date: DateTime::default(),
            event_details: EventDetails {
                event_type: EventType::Talk,
                event_recurrence: EventRecurrence::OneOff,
                event_status: EventStatus::Confirmed,
                age_range: None,
                image_url: None,
                contact_details: ContactDetails {
                    full_name: "Zayd McArdle".to_owned(),
                    title: None,
                    phone_number: "07123456789".to_string(),
                    email: None,
                },
            },
        }];
        let test_cases = [
            TestCase {
                description: "When retrieving events fails on in-memory repository and main repository, I should get an error",
                expected_in_memory_db_response: Err(GetEventsRepositoryError::UnableToGetEvents),
                expected_db_response: Some(Err(GetEventsRepositoryError::UnableToGetEvents)),
                expected_result: Err(GetEventsServiceError::UnableToGetEventsFromRepository(
                    GetEventsRepositoryError::UnableToGetEvents,
                )),
            },
            TestCase {
                description: "When events not found on either repository, I should get a not found error",
                expected_in_memory_db_response: Err(GetEventsRepositoryError::EventsNotFound),
                expected_db_response: Some(Err(GetEventsRepositoryError::EventsNotFound)),
                expected_result: Err(GetEventsServiceError::UnableToGetEventsFromRepository(
                    GetEventsRepositoryError::EventsNotFound,
                )),
            },
            TestCase {
                description: "When events not found in in-memory repository but is found in main repository, I should get no error",
                expected_in_memory_db_response: Err(GetEventsRepositoryError::EventsNotFound),
                expected_db_response: Some(Ok(events.clone())),
                expected_result: Ok(events.clone()),
            },
            TestCase {
                description: "When events found in in-memory repository",
                expected_in_memory_db_response: Ok(events.clone()),
                expected_db_response: None,
                expected_result: Ok(events),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_repository = MockEventsRepository::new();
            let mut mock_in_memory_repository = MockEventsRepository::new();

            mock_in_memory_repository
                .expect_get_events()
                .return_once(move || test_case.expected_in_memory_db_response);
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_repository
                    .expect_get_events()
                    .return_once(move || expected_db_response);
            }

            let service = new_event_retrieval_service(
                Arc::new(mock_repository),
                Arc::new(mock_in_memory_repository),
            );
            let actual_result = service.get_events().await;
            assert!(matches!(test_case.expected_result, actual_result));
        }
    }
}
