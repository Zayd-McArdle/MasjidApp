#[derive(Debug)]
pub enum InsertNewUserError {
    UserExists,
    DatabaseError,
}
