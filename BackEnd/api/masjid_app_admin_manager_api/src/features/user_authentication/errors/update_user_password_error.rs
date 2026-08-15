#[derive(Debug)]
pub enum UpdateUserPasswordError {
    UserDoesNotExist,
    DatabaseError,
}
