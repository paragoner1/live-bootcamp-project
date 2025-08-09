use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn login_returns_200() {
    let app = TestApp::new().await;

    // First create a user
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let signup_response = app.post_signup(&signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    // Then login with the same credentials
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);
}