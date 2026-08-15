#[derive(Debug)]
pub enum GetEventsRepositoryError {
    EventsNotFound,
    UnableToGetEvents,
}
