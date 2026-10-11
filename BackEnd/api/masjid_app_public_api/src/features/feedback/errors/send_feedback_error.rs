use crate::features::feedback::errors::insert_feedback_repository_error::InsertFeedbackRepositoryError;

pub enum SendFeedbackError {
    RepositoryError(InsertFeedbackRepositoryError),
}

impl From<InsertFeedbackRepositoryError> for SendFeedbackError {
    fn from(value: InsertFeedbackRepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
