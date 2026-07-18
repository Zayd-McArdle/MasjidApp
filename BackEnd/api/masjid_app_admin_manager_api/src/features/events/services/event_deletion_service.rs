use crate::features::events::repositories::EventsAdminRepository;
use crate::features::events::services::errors::event_deletion_error::EventDeletionError;
use async_trait::async_trait;
use masjid_app_api_library::features::events::services::event_service_impl::EventServiceImpl;
use masjid_app_api_library::new_event_service;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait EventDeletionService: Send + Sync {
    async fn delete_event(&self, id: i32) -> Result<(), EventDeletionError>;
}
new_event_service!(
    new_event_deletion_service,
    EventDeletionService,
    EventsAdminRepository
);
#[async_trait]
impl EventDeletionService for EventServiceImpl<dyn EventsAdminRepository> {
    async fn delete_event(&self, id: i32) -> Result<(), EventDeletionError> {
        let delete_event_in_memory_repository_result = self
            .common
            .in_memory_repository
            .delete_event_by_id(&id)
            .await
            .map_err(EventDeletionError::from);
        if let Err(in_memory_repository_error) = delete_event_in_memory_repository_result {
            tracing::warn!(in_memory_repository_error = ?in_memory_repository_error, "failure to delete event from in-memory repository");
        }
        self.common
            .repository
            .delete_event_by_id(&id)
            .await
            .map(move |_image_url| {
                //TODO - Add file IO implementation for storing/removing images from the web server
            })
            .map_err(EventDeletionError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::events::errors::DeleteEventError;
    use crate::features::events::errors::UpsertEventError;
    use crate::features::events::repositories::EventsAdminRepository;
    use crate::features::events::services::errors::event_deletion_error::EventDeletionError;
    use async_trait::async_trait;
    use masjid_app_api_library::features::events::errors::GetEventsRepositoryError;
    use masjid_app_api_library::features::events::models::Event;
    use masjid_app_api_library::features::events::models::EventDTO;
    use masjid_app_api_library::features::events::repositories::EventsRepository;
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
    async fn test_event_deletion_service_delete_event() {
        struct TestCase {
            description: &'static str,
            id: i32,
            expected_db_response: Result<Option<String>, DeleteEventError>,
            expected_result: Result<(), EventDeletionError>,
        }

        let test_cases = [
            TestCase {
                description: "When deletion fails, I should get an error",
                id: 0,
                expected_db_response: Err(DeleteEventError::UnableToDeleteEvent),
                expected_result: Err(EventDeletionError::RepositoryError(
                    DeleteEventError::UnableToDeleteEvent,
                )),
            },
            TestCase {
                description: "When event not found, I should get an error",
                id: 0,
                expected_db_response: Err(DeleteEventError::EventNotFound),
                expected_result: Err(EventDeletionError::RepositoryError(
                    DeleteEventError::EventNotFound,
                )),
            },
            TestCase {
                description: "When deletion successful, I should get no error",
                id: 0,
                expected_db_response: Ok(None),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_in_memory_repository = MockEventsAdminRepository::new();
            let mut mock_repository = MockEventsAdminRepository::new();

            mock_in_memory_repository
                .expect_delete_event_by_id()
                .return_once(move |_| Ok(None));
            mock_repository
                .expect_delete_event_by_id()
                .return_once(move |_| test_case.expected_db_response);
            let service = new_event_deletion_service(
                Arc::new(mock_repository),
                Arc::new(mock_in_memory_repository),
            );
            let actual_result = service.delete_event(test_case.id);
            assert!(matches!(test_case.expected_result, actual_result));
        }
    }
}
