#[derive(Debug, Eq, PartialEq)]
pub enum DeleteFeedbackRepositoryError {
    FeedbackNotFound,
    UnableToDeleteFeedback,
}
