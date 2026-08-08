#[derive(Clone, PartialEq, Debug)]
pub enum UpdateUserPasswordError {
    UserDoesNotExist,
    DatabaseError,
}
