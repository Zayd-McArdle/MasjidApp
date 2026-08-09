#[derive(Debug)]
pub enum InsertEventError {
    EventAlreadyExists,
    UnableToInsertEvent,
}
