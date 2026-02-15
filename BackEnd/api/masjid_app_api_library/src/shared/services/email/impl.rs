use crate::shared::services::email::email_message::EmailMessage;
use crate::shared::services::email::errors::SendEmailError;
use crate::shared::services::email::r#trait::EmailService;
use async_trait::async_trait;

pub(super) struct EmailServiceImpl;

#[async_trait]
impl EmailService for EmailServiceImpl {
    async fn send_email(&self, message: EmailMessage) -> Result<(), SendEmailError> {
        todo!()
    }
}
