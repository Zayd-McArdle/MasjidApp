#[derive(Debug, Eq, PartialEq)]
pub enum GetFeedbackRepositoryError {
    NotFound,
    UnableToRetrieveFeedback,
}
