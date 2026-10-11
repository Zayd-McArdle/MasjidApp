use crate::features::feedback::repositories::errors::get_feedback_repository_error::GetFeedbackRepositoryError;

pub enum GetFeedbackServiceError {
    RepositoryError(GetFeedbackRepositoryError),
}
