#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DeleteEventError {
    UnableToDeleteEvent,
    EventNotFound,
}
