use crate::features::events::repositories::errors::delete_event_error::DeleteEventError;

#[derive(Debug, PartialEq)]
pub enum EventDeletionError {
    RepositoryError(DeleteEventError),
    UnableToDeleteImagesRelatedToEvent,
}
impl From<DeleteEventError> for EventDeletionError {
    #[inline]
    fn from(value: DeleteEventError) -> Self {
        Self::RepositoryError(value)
    }
}
