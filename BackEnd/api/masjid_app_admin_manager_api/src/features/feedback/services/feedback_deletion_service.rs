use crate::features::feedback::repositories::errors::delete_feedback_repository_error::DeleteFeedbackRepositoryError;
use crate::features::feedback::repositories::feedback_admin_repository::FeedbackAdminRepository;
use crate::features::feedback::services::errors::delete_feedback_service_error::DeleteFeedbackServiceError;
use async_trait::async_trait;
use masjid_app_api_library::features::feedback::services::feedback_service::FeedbackServiceImpl;
use masjid_app_api_library::new_feedback_service;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use masjid_app_api_library::shared::services::file_management::file_deletion::service::FileDeletionService;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait FeedbackDeletionService: Send + Sync {
    async fn delete_feedback(&self, ids: &[u64]) -> Result<(), DeleteFeedbackServiceError>;
}

new_feedback_service!(
    new_feedback_deletion_service,
    FeedbackDeletionService,
    FeedbackAdminRepository,
    FileDeletionService
);

#[async_trait]
impl FeedbackDeletionService
    for FeedbackServiceImpl<dyn FeedbackAdminRepository, dyn FileDeletionService>
{
    async fn delete_feedback(&self, ids: &[u64]) -> Result<(), DeleteFeedbackServiceError> {
        match self
            .common
            .in_memory_repository
            .delete_feedback_by_id(ids)
            .await
        {
            Ok(()) | Err(DeleteFeedbackRepositoryError::FeedbackNotFound) => {
                self.common.repository.delete_feedback_by_id(ids).await?
            }
            Err(err) => return Err(err.into()),
        };
        self.image_management_service
            .delete(ids)
            .await
            .map_err(DeleteFeedbackServiceError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::feedback::repositories::errors::delete_feedback_repository_error::DeleteFeedbackRepositoryError;
    use crate::features::feedback::repositories::feedback_admin_repository::MockFeedbackAdminRepository;

    #[tokio::test]
    async fn test_feedback_deletion_service_delete_feedback() {
        struct TestCase {
            description: &'static str,
            mock_in_memory_repository_result: Result<(), DeleteFeedbackRepositoryError>,
            mock_repository_result: Option<Result<(), DeleteFeedbackRepositoryError>>,
            expected_result: Result<(), DeleteFeedbackServiceError>,
        }
        let ids = [0, 1, 2, 3];
        let test_cases = [
            TestCase {
                description: "When feedback is not found in both repositories, I expect an error",
                mock_in_memory_repository_result: Err(
                    DeleteFeedbackRepositoryError::FeedbackNotFound,
                ),
                mock_repository_result: Some(Err(DeleteFeedbackRepositoryError::FeedbackNotFound)),
                expected_result: Err(DeleteFeedbackServiceError::RepositoryError(
                    DeleteFeedbackRepositoryError::FeedbackNotFound,
                )),
            },
            TestCase {
                description: "When feedback deletion fails in in-memory repository, I expect an error",
                mock_in_memory_repository_result: Err(
                    DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
                ),
                mock_repository_result: None,
                expected_result: Err(DeleteFeedbackServiceError::RepositoryError(
                    DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
                )),
            },
            TestCase {
                description: "When feedback is not found in in-memory repository but fails to delete in main repository, I expect an error",
                mock_in_memory_repository_result: Err(
                    DeleteFeedbackRepositoryError::FeedbackNotFound,
                ),
                mock_repository_result: Some(Err(
                    DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
                )),
                expected_result: Err(DeleteFeedbackServiceError::RepositoryError(
                    DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
                )),
            },
            TestCase {
                description: "When feedback deletion succeeds in in-memory repository but feedback not found in main repository, I expect an error",
                mock_in_memory_repository_result: Ok(()),
                mock_repository_result: Some(Err(DeleteFeedbackRepositoryError::FeedbackNotFound)),
                expected_result: Err(DeleteFeedbackServiceError::RepositoryError(
                    DeleteFeedbackRepositoryError::FeedbackNotFound,
                )),
            },
            TestCase {
                description: "When feedback deletion succeeds in in-memory repository but fails in main repository, I expect an error",
                mock_in_memory_repository_result: Ok(()),
                mock_repository_result: Some(Err(
                    DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
                )),
                expected_result: Err(DeleteFeedbackServiceError::RepositoryError(
                    DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
                )),
            },
            TestCase {
                description: "When feedback is not found in in-memory repository but succeeds in main repository, I expect no error",
                mock_in_memory_repository_result: Err(
                    DeleteFeedbackRepositoryError::FeedbackNotFound,
                ),
                mock_repository_result: Some(Ok(())),
                expected_result: Ok(()),
            },
            TestCase {
                description: "When feedback deletion fails in in-memory repository but feedback is not found in main repository, I expect an error",
                mock_in_memory_repository_result: Err(
                    DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
                ),
                mock_repository_result: None,
                expected_result: Err(DeleteFeedbackServiceError::RepositoryError(
                    DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
                )),
            },
            TestCase {
                description: "When feedback deletions succeeds in both repositories, I expect no error",
                mock_in_memory_repository_result: Ok(()),
                mock_repository_result: Some(Ok(())),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_in_memory_repository = MockFeedbackAdminRepository::new();
            let mut mock_repository = MockFeedbackAdminRepository::new();

            mock_in_memory_repository
                .expect_delete_feedback_by_id()
                .return_once(move |_| test_case.mock_in_memory_repository_result);
            if let Some(mock_repository_result) = test_case.mock_repository_result {
                mock_repository
                    .expect_delete_feedback_by_id()
                    .return_once(move |_| mock_repository_result);
            }

            let service = new_feedback_deletion_service(
                Arc::new(mock_repository),
                Arc::new(mock_in_memory_repository),
                Arc::new(),
            );

            let actual_result = service.delete_feedback(&ids).await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
