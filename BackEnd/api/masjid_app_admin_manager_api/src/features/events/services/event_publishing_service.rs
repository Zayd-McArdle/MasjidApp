use crate::features::events::repositories::EventsAdminRepository;
use crate::features::events::services::errors::event_publishing_error::EventPublishingError;
use async_trait::async_trait;
use masjid_app_api_library::features::events::models::{Event, EventDTO};
use masjid_app_api_library::features::events::repositories::EventsRepository;
use masjid_app_api_library::features::events::services::event_service_impl::EventServiceImpl;
use masjid_app_api_library::new_event_service;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait EventPublishingService: Send + Sync {
    async fn publish_event(&self, event: EventDTO) -> Result<(), EventPublishingError>;
}

new_event_service!(
    new_event_publishing_service,
    EventPublishingService,
    EventsAdminRepository
);

#[async_trait]
impl EventPublishingService for EventServiceImpl<dyn EventsAdminRepository> {
    async fn publish_event(&self, event: EventDTO) -> Result<(), EventPublishingError> {
        let event: Event = event.into();
        if let Err(upsert_error) = self.common.in_memory_repository.upsert_event(&event).await {
            tracing::warn!(in_memory_upsertion_error = ?upsert_error, "upserting event into in-memory repository failed");
        }
        self.common
            .repository
            .upsert_event(&event)
            .await
            .map_err(EventPublishingError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::events::errors::{DeleteEventError, UpsertEventError};
    use async_trait::async_trait;
    use chrono::DateTime;
    use masjid_app_api_library::features::events::errors::GetEventsRepositoryError;
    use masjid_app_api_library::features::events::models::{
        EventDetails, EventRecurrence, EventStatus, EventType,
    };
    use masjid_app_api_library::features::events::repositories::EventsRepository;
    use masjid_app_api_library::shared::types::contact_details::ContactDetails;
    use mockall::mock;

    mock!(
        pub EventsAdminRepository {}

        #[async_trait]
        impl EventsRepository for EventsAdminRepository {
            async fn get_events(&self) -> Result<Vec<EventDTO>, GetEventsRepositoryError>;
        }
        #[async_trait]
        impl EventsAdminRepository for EventsAdminRepository {
            async fn upsert_event(&self, event: &Event) -> Result<(), UpsertEventError>;
            async fn delete_event_by_id(&self, event_id: &i32) -> Result<Option<String>, DeleteEventError>;
        }
    );

    #[tokio::test]
    async fn test_event_publishing_service_publish_event() {
        struct TestCase {
            description: &'static str,
            event_dto: EventDTO,
            expected_db_response: Result<(), UpsertEventError>,
            expected_result: Result<(), EventPublishingError>,
        }
        let event_dto = EventDTO {
            id: 0,
            title: "some title".to_string(),
            description: None,
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
                    phone_number: "07123456789".to_owned(),
                    email: None,
                },
            },
        };
        let test_cases = [
            TestCase {
                description: "When upsertion fails, I should receive an error",
                event_dto: event_dto.clone(),
                expected_db_response: Err(UpsertEventError::UnableToUpsertEvent),
                expected_result: Err(EventPublishingError::RepositoryError(
                    UpsertEventError::UnableToUpsertEvent,
                )),
            },
            TestCase {
                description: "When upsertion succeeds, I should receive no error",
                event_dto,
                expected_db_response: Ok(()),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_in_memory_repository = MockEventsAdminRepository::new();
            let mut mock_repository = MockEventsAdminRepository::new();

            mock_in_memory_repository
                .expect_upsert_event()
                .return_once(move |_| Ok(()));
            mock_repository
                .expect_upsert_event()
                .return_once(move |_| test_case.expected_db_response);

            let service = new_event_publishing_service(
                Arc::new(mock_repository),
                Arc::new(mock_in_memory_repository),
            );

            let actual_result = service.publish_event(test_case.event_dto).await;
            assert!(matches!(test_case.expected_result, actual_result));
        }
    }
}
