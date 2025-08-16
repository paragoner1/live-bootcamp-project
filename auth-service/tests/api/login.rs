use crate::helpers::{get_random_email, TestApp};

//...

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    // First create a user with 2FA enabled
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let signup_response = app.post_signup(&signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    // Then login with the same credentials
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_login(&login_body).await;

    // Should return 206 (Partial Content) for 2FA users
    assert_eq!(response.status().as_u16(), 206);

    // Parse the response body to verify it contains the expected 2FA message
    let response_body = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(response_body["message"], "2FA required");
    
    // Verify loginAttemptId is a valid UUID (not hard-coded "123456")
    let login_attempt_id = response_body["loginAttemptId"].as_str().unwrap();
    assert!(uuid::Uuid::parse_str(login_attempt_id).is_ok(), "loginAttemptId should be a valid UUID");
}


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