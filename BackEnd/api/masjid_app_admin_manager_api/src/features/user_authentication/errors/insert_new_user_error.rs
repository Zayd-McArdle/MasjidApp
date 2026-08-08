#[derive(Debug, Clone)]
pub enum InsertNewUserError {
    UserExists,
    DatabaseError,
}
