use auth_service::{
    domain::{Email, EmailClient},
    services::mock_email_client::MockEmailClient,
};

#[tokio::main]
async fn main() {
    let email_client = MockEmailClient;
    
    let email = Email::parse("test@example.com".to_string()).unwrap();
    
    println!("Testing email client...");
    
    let result = email_client
        .send_email(
            &email,
            "Your 2FA Code",
            "Your verification code is: 123456",
        )
        .await;
    
    match result {
        Ok(()) => println!("✅ Email sent successfully!"),
        Err(e) => println!("❌ Email failed: {}", e),
    }
}
