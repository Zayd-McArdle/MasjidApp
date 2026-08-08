#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetEventsRepositoryError {
    EventsNotFound,
    UnableToGetEvents,
}
