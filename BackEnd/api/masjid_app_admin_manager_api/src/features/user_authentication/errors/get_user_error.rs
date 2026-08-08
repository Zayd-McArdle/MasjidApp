#[derive(Clone, Debug, PartialEq)]
pub enum GetUserError {
    NotFound,
    DatabaseError,
}
