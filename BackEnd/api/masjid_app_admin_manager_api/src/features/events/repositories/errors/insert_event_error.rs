#[derive(Debug, PartialEq)]
pub enum InsertEventError {
    EventAlreadyExists,
    UnableToInsertEvent,
}
