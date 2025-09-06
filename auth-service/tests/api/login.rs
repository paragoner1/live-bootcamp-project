use crate::helpers::{get_random_email, TestApp};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

//...

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let mut app = TestApp::new().await;

    // First create a user with 2FA enabled
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let signup_response = app.post_signup(&signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    // Define an expectation for the mock server
    Mock::given(path("/email")) // Expect an HTTP request to the "/email" path
        .and(method("POST")) // Expect the HTTP method to be POST
        .respond_with(ResponseTemplate::new(200)) // Respond with an HTTP 200 OK status
        .expect(1) // Expect this request to be made exactly once
        .mount(&app.email_server) // Mount this expectation on the mock email server
        .await; // Await the asynchronous operation to ensure the mock server is set up before proceeding

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

    app.clean_up().await;
}


#[tokio::test]
async fn login_returns_200() {
    let mut app = TestApp::new().await;

    // First create a user
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let signup_response = app.post_signup(&signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    // No email mock needed since this user doesn't require 2FA

    // Then login with the same credentials
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    app.clean_up().await;
}