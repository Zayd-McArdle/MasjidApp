use crate::features::feedback::repositories::errors::delete_feedback_repository_error::DeleteFeedbackRepositoryError;
use masjid_app_api_library::shared::services::file_management::file_deletion::error::FileDeletionError;

#[derive(Debug, Eq, PartialEq)]
pub enum DeleteFeedbackServiceError {
    RepositoryError(DeleteFeedbackRepositoryError),
    ImageDeletionError(FileDeletionError),
}

impl From<DeleteFeedbackRepositoryError> for DeleteFeedbackServiceError {
    fn from(value: DeleteFeedbackRepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl From<FileDeletionError> for DeleteFeedbackServiceError {
    fn from(value: FileDeletionError) -> Self {
        Self::ImageDeletionError(value)
    }
}
