use crate::features::events::errors::GetEventsRepositoryError;

pub enum GetEventsServiceError {
    UnableToGetEventsFromRepository(GetEventsRepositoryError),
}

impl From<GetEventsRepositoryError> for GetEventsServiceError {
    #[inline]
    fn from(value: GetEventsRepositoryError) -> Self {
        Self::UnableToGetEventsFromRepository(value)
    }
}
