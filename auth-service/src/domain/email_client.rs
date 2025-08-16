use super::Email;

// This trait represents the interface all concrete email clients should implement
#[async_trait::async_trait]
pub trait EmailClient {
    async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> Result<(), String>;
}
