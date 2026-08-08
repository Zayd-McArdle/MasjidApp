#[derive(Debug, PartialEq)]
pub enum UpdateEventError {
    EventNotFound,
    UnableToUpdateEvent,
}
