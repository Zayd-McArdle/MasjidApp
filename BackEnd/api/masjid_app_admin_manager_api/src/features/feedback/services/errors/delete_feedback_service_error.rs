use crate::features::feedback::repositories::errors::delete_feedback_repository_error::DeleteFeedbackRepositoryError;

#[derive(Debug, Eq, PartialEq)]
pub enum DeleteFeedbackServiceError {
    RepositoryError(DeleteFeedbackRepositoryError),
}

impl From<DeleteFeedbackRepositoryError> for DeleteFeedbackServiceError {
    fn from(value: DeleteFeedbackRepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
