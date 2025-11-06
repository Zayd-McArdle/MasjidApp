#[derive(Clone, Debug, PartialEq)]
pub enum LoginError {
    InvalidCredentials,
    UnableToLogin,
}

#[derive(Clone)]
pub enum RegistrationError {
    UserAlreadyRegistered,
    FailedToRegister,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ResetPasswordError {
    UserDoesNotExist,
    FailedToResetUserPassword,
}