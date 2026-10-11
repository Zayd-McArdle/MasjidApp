#[derive(Debug)]
pub enum DeleteFeedbackByIdRepositoryError {
    FeedbackIdNotFound,
    UnableToDeleteFeedbackId,
}
