#[derive(Debug)]
pub enum DeleteEventError {
    UnableToDeleteEvent,
    EventNotFound,
}
