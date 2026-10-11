#[derive(Debug, Eq, PartialEq)]
pub enum DeleteFeedbackRepositoryError {
    FeedbackNotFound(u64),
    UnableToDeleteFeedback,
}
