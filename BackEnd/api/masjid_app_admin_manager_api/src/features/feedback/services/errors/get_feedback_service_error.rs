use crate::features::feedback::repositories::errors::get_feedback_repository_error::GetFeedbackRepositoryError;

#[derive(Debug, Eq, PartialEq)]
pub enum GetFeedbackServiceError {
    RepositoryError(GetFeedbackRepositoryError),
}
