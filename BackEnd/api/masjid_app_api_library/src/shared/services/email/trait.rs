use crate::shared::services::email::email_message::EmailMessage;
use crate::shared::services::email::email_provider::EmailProvider;
use crate::shared::services::email::errors::SendEmailError;
use crate::shared::services::email::r#impl::EmailServiceImpl;
use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_email(&self, message: EmailMessage) -> Result<(), SendEmailError>;
}

pub fn new_email_service(email_provider: EmailProvider) -> Arc<dyn EmailService> {
    match email_provider {
        EmailProvider::Lettre => Arc::new(EmailServiceImpl),
    }
}
