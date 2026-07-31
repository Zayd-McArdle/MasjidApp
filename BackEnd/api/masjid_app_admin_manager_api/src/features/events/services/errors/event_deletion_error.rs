use crate::features::events::errors::DeleteEventError;

#[derive(Debug)]
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
