use crate::features::events::repositories::errors::insert_event_error::InsertEventError;
use crate::features::events::repositories::errors::update_event_error::UpdateEventError;

#[derive(Debug)]
pub enum UpsertEventError {
    InsertError(InsertEventError),
    UpdateError(UpdateEventError),
    UnableToUpsertEvent,
}
