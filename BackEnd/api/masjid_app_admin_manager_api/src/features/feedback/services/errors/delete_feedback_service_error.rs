use crate::features::feedback::repositories::errors::{
    delete_feedback_by_id_repository_error::DeleteFeedbackByIdRepositoryError,
    delete_feedback_repository_error::DeleteFeedbackRepositoryError,
};

#[derive(Debug, Eq, PartialEq)]
pub enum DeleteFeedbackServiceError {
    RepositorySingleDeletionError(DeleteFeedbackByIdRepositoryError),
    RepositoryMultiDeletionError(DeleteFeedbackRepositoryError),
}
