#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetEventsError {
    EventsNotFound,
    UnableToGetEvents,
}