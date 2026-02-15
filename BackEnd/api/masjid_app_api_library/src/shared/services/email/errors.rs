#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SendEmailError {
    InvalidEmail,
    EmailBounceBack,
}
