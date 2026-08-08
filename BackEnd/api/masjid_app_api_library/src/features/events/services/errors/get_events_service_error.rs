use crate::features::events::repositories::errors::get_events_repository_error::GetEventsRepositoryError;

#[derive(Debug, PartialEq)]
pub enum GetEventsServiceError {
    UnableToGetEventsFromRepository(GetEventsRepositoryError),
}

impl From<GetEventsRepositoryError> for GetEventsServiceError {
    #[inline]
    fn from(value: GetEventsRepositoryError) -> Self {
        Self::UnableToGetEventsFromRepository(value)
    }
}
