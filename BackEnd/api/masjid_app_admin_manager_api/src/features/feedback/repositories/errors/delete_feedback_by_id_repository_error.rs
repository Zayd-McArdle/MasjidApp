#[derive(Debug, Eq, PartialEq)]
pub enum DeleteFeedbackByIdRepositoryError {
    FeedbackIdNotFound,
    UnableToDeleteFeedbackId,
}
