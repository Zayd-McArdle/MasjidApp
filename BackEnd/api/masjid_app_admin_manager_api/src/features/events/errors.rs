#[derive(Debug)]
pub enum InsertEventError {
    EventAlreadyExists,
    UnableToInsertEvent,
}
#[derive(Debug)]
pub enum UpdateEventError {
    EventNotFound,
    UnableToUpdateEvent,
}
#[derive(Debug)]
pub enum UpsertEventError {
    InsertError(InsertEventError),
    UpdateError(UpdateEventError),
    UnableToUpsertEvent,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DeleteEventError {
    UnableToDeleteEvent,
    EventNotFound,
}
