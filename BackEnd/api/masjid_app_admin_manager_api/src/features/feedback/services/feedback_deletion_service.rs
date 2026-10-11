use crate::features::feedback::models::feedback_deletion_mode::FeedbackDeletionMode;
use crate::features::feedback::repositories::feedback_admin_repository::FeedbackAdminRepository;
use crate::features::feedback::services::errors::delete_feedback_service_error::DeleteFeedbackServiceError;
use async_trait::async_trait;
use masjid_app_api_library::new_service;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait FeedbackDeletionService: Send + Sync {
    async fn delete_feedback(&self, ids: &[u64]) -> Result<(), DeleteFeedbackServiceError>;
}

new_service!(
    new_feedback_deletion_service,
    FeedbackDeletionService,
    FeedbackAdminRepository
);

#[async_trait]
impl FeedbackDeletionService for CommonServiceImpl<dyn FeedbackAdminRepository> {
    async fn delete_feedback(&self, ids: &[u64]) -> Result<(), DeleteFeedbackServiceError> {
        self.repository
            .delete_feedback_by_id(ids)
            .await
            .and(self.in_memory_repository.delete_feedback_by_id(ids).await)
            .map_err(DeleteFeedbackServiceError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::feedback::repositories::errors::delete_feedback_by_id_repository_error::DeleteFeedbackByIdRepositoryError;
    use crate::features::feedback::repositories::errors::delete_feedback_repository_error::DeleteFeedbackRepositoryError;
    use crate::features::feedback::repositories::feedback_admin_repository::MockFeedbackAdminRepository;
    use crate::features::feedback::services::feedback_deletion_service::tests::MockRepositoryResult::{Multi, Single};

    enum MockRepositoryResult {
        Single(
            // In-memory repository result
            Result<(), DeleteFeedbackByIdRepositoryError>,
            // Main repository result
            Result<(), DeleteFeedbackByIdRepositoryError>,
        ),
        Multi(
            // In-memory repository result
            Result<(), DeleteFeedbackRepositoryError>,
            // Main repository result
            Result<(), DeleteFeedbackRepositoryError>,
        ),
    }
    #[tokio::test]
    async fn test_feedback_deletion_service_delete_feedback() {
        struct TestCase {
            description: &'static str,
            mock_repository_result: MockRepositoryResult,
            expected_result: Result<(), DeleteFeedbackServiceError>,
        }
        let test_cases = [
            TestCase {
                description: "When in single deletion mode and feedback is not found, I expect an error",
                mock_repository_result: Single(
                    Err(DeleteFeedbackByIdRepositoryError::FeedbackIdNotFound),
                    Err(DeleteFeedbackByIdRepositoryError::FeedbackIdNotFound),
                ),
                expected_result: Err(DeleteFeedbackServiceError::RepositorySingleDeletionError(
                    DeleteFeedbackByIdRepositoryError::FeedbackIdNotFound,
                )),
            },
            TestCase {
                description: "When in single deletion mode, and feedback deletion fails, I expect an error",
                mock_repository_result: Single(
                    Err(DeleteFeedbackByIdRepositoryError::UnableToDeleteFeedbackId),
                    Err(DeleteFeedbackByIdRepositoryError::UnableToDeleteFeedbackId),
                ),
                expected_result: Err(DeleteFeedbackServiceError::RepositorySingleDeletionError(
                    DeleteFeedbackByIdRepositoryError::UnableToDeleteFeedbackId,
                )),
            },
            TestCase {
                description: "When in single deletion mode, and feedback deletions succeeds, I expect no error",
                mock_repository_result: Single(Ok(()), Ok(())),
                expected_result: Ok(()),
            },
            TestCase {
                description: "When in multi-deletion mode, and feedback not found based on filtered criteria, I expect an error",
                mock_repository_result: Multi(
                    Err(DeleteFeedbackRepositoryError::FeedbackNotFound),
                    Err(DeleteFeedbackRepositoryError::FeedbackNotFound),
                ),
                expected_result: Err(DeleteFeedbackServiceError::RepositoryMultiDeletionError(
                    DeleteFeedbackRepositoryError::FeedbackNotFound,
                )),
            },
            TestCase {
                description: "When in multi-deletion mode, and feedback deletion fails, I expect an error",
                mock_repository_result: Multi(
                    Err(DeleteFeedbackRepositoryError::UnableToDeleteFeedback),
                    Err(DeleteFeedbackRepositoryError::UnableToDeleteFeedback),
                ),
                expected_result: Err(DeleteFeedbackServiceError::RepositoryMultiDeletionError(
                    DeleteFeedbackRepositoryError::UnableToDeleteFeedback,
                )),
            },
            TestCase {
                description: "When in multi-deletion mode and feedback deletion succeeds, I expect no error",
                mock_repository_result: Multi(Ok(()), Ok(())),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_repository = MockFeedbackAdminRepository::new();
            let mut mock_in_memory_repository = MockFeedbackAdminRepository::new();

            let deletion_mode = match test_case.mock_repository_result {
                Single(mock_in_memory_repository_result, mock_repository_result) => {
                    mock_in_memory_repository
                        .expect_delete_feedback_by_id()
                        .return_once(move |_| mock_in_memory_repository_result);
                    mock_repository
                        .expect_delete_feedback_by_id()
                        .return_once(move |_| mock_repository_result);
                    FeedbackDeletionMode::Single(1)
                }
            };

            let service = CommonServiceImpl::<dyn FeedbackAdminRepository> {
                in_memory_repository: Arc::new(mock_in_memory_repository),
                repository: Arc::new(mock_repository),
            };

            let actual_result = service.delete_feedback(deletion_mode).await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
