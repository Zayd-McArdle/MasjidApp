use crate::features::events::errors::DeleteEventError;
use crate::features::events::services::errors::event_deletion_error::EventDeletionError::RepositoryError;

#[derive(Debug)]
pub enum EventDeletionError {
    RepositoryError(DeleteEventError),
    UnableToDeleteImagesRelatedToEvent,
}
impl From<DeleteEventError> for EventDeletionError {
    fn from(value: DeleteEventError) -> Self {
        RepositoryError(value)
    }
}
