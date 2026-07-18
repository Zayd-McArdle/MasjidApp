use crate::features::events::errors::UpsertEventError;

#[derive(Debug)]
pub enum EventPublishingError {
    UnableToSaveImage,
    RepositoryError(UpsertEventError),
}
impl From<UpsertEventError> for EventPublishingError {
    fn from(value: UpsertEventError) -> Self {
        Self::RepositoryError(value)
    }
}
