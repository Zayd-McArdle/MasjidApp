use crate::features::events::repositories::errors::upsert_event_error::UpsertEventError;

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
