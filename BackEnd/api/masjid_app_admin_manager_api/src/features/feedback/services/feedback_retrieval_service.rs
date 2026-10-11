use crate::features::feedback::models::filters::get_feedback_filter::GetFeedbackFilter;
use crate::features::feedback::repositories::feedback_admin_repository::FeedbackAdminRepository;
use crate::features::feedback::services::errors::get_feedback_service_error::GetFeedbackServiceError;
use async_trait::async_trait;
use masjid_app_api_library::features::feedback::models::feedback_dto::FeedbackDTO;
use masjid_app_api_library::new_service;
use masjid_app_api_library::shared::common_service_impl::CommonServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait FeedbackRetrievalService: Send + Sync {
    async fn retrieve_feedback(
        &self,
        filter: GetFeedbackFilter,
    ) -> Result<Vec<FeedbackDTO>, GetFeedbackServiceError>;
}

new_service!(
    new_feedback_retrieval_service,
    FeedbackRetrievalService,
    FeedbackAdminRepository
);

#[async_trait]
impl FeedbackRetrievalService for CommonServiceImpl<dyn FeedbackAdminRepository> {
    async fn retrieve_feedback(
        &self,
        filter: GetFeedbackFilter,
    ) -> Result<Vec<FeedbackDTO>, GetFeedbackServiceError> {
        match self.in_memory_repository.get_feedback(&filter).await {
            Ok(feedback) => Ok(feedback),
            Err(_) => self
                .repository
                .get_feedback(&filter)
                .await
                .map_err(move |err| GetFeedbackServiceError::RepositoryError(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::feedback::models::filters::get_feedback_filter::GetFeedbackFilter;
    use crate::features::feedback::repositories::errors::get_feedback_repository_error::GetFeedbackRepositoryError;
    use crate::features::feedback::repositories::feedback_admin_repository::MockFeedbackAdminRepository;
    use crate::features::feedback::services::errors::get_feedback_service_error::GetFeedbackServiceError;
    use masjid_app_api_library::features::feedback::models::concern_type::ConcernType;
    use masjid_app_api_library::features::feedback::models::feedback_dto::FeedbackDTO;
    use masjid_app_api_library::features::feedback::models::feedback_type::FeedbackType;

    #[tokio::test]
    async fn feedback_retrieval_service_retrieve_feedback() {
        struct TestCase {
            description: &'static str,
            filter: GetFeedbackFilter,
            mock_in_memory_repository_result: Result<Vec<FeedbackDTO>, GetFeedbackRepositoryError>,
            mock_repository_result: Result<Vec<FeedbackDTO>, GetFeedbackRepositoryError>,
            expected_result: Result<Vec<FeedbackDTO>, GetFeedbackServiceError>,
        }
        let feedback_dtos = vec![FeedbackDTO {
            id: 0,
            title: "".to_string(),
            feedback_type: FeedbackType::Concern(ConcernType::NeighbourDisruption),
            details: "".to_string(),
            images: None,
            date: chrono::DateTime::default(),
        }];

        let test_cases = [
            TestCase {
                description: "When no feedback was found, I expect an error",
                filter: GetFeedbackFilter::default(),
                mock_in_memory_repository_result: Err(GetFeedbackRepositoryError::NotFound),
                mock_repository_result: Err(GetFeedbackRepositoryError::NotFound),
                expected_result: Err(GetFeedbackServiceError::RepositoryError(
                    GetFeedbackRepositoryError::NotFound,
                )),
            },
            TestCase {
                description: "When feedback retrieval fails, I expect an error",
                filter: GetFeedbackFilter::default(),
                mock_in_memory_repository_result: Err(
                    GetFeedbackRepositoryError::UnableToRetrieveFeedback,
                ),
                mock_repository_result: Err(GetFeedbackRepositoryError::UnableToRetrieveFeedback),
                expected_result: Err(GetFeedbackServiceError::RepositoryError(
                    GetFeedbackRepositoryError::UnableToRetrieveFeedback,
                )),
            },
            TestCase {
                description: "When feedbacm retrieval succeeds, I expect no error",
                filter: GetFeedbackFilter::default(),
                mock_in_memory_repository_result: Ok(feedback_dtos.clone()),
                mock_repository_result: Ok((feedback_dtos.clone())),
                expected_result: Ok((feedback_dtos)),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_in_memory_repository = MockFeedbackAdminRepository::new();
            let mut mock_repository = MockFeedbackAdminRepository::new();

            mock_in_memory_repository
                .expect_get_feedback()
                .return_once(move |_| test_case.mock_in_memory_repository_result);
            mock_repository
                .expect_get_feedback()
                .return_once(move |_| test_case.mock_repository_result);

            let service = CommonServiceImpl::<dyn FeedbackAdminRepository> {
                in_memory_repository: Arc::new(mock_in_memory_repository),
                repository: Arc::new(mock_repository),
            };

            let actual_result = service.retrieve_feedback(test_case.filter).await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
