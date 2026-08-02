#[derive(Clone, Debug, PartialEq)]
pub enum GetUserError {
    NotFound,
    DatabaseError,
}

#[derive(Debug, Clone)]
pub enum InsertNewUserError {
    UserExists,
    DatabaseError,
}

#[derive(Clone, PartialEq, Debug)]
pub enum UpdateUserPasswordError {
    UserDoesNotExist,
    DatabaseError,
}
