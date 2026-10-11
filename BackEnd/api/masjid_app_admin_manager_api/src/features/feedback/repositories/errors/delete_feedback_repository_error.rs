#[derive(Debug)]
pub enum DeleteFeedbackRepositoryError {
    FeedbackNotFound,
    UnableToDeleteFeedback,
}
