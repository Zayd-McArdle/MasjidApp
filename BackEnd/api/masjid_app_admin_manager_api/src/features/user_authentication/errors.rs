#[derive(Clone, Debug, PartialEq)]
pub enum GetUserError {
    NotFound,
    DatabaseError,
}

#[derive(Clone)]
pub enum InsertNewUserError {
    UserExists,
    DatabaseError,
}

#[derive(Clone, PartialEq, Debug)]
pub enum UpdateUserPasswordError {
    UserDoesNotExist,
    DatabaseError,
}