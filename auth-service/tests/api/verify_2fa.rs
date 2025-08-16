use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
    ErrorResponse,
};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let test_cases = [
        serde_json::json!({
            "email": "test@example.com",
            "loginAttemptId": "12345678-1234-1234-1234-123456789012"
        }),
        serde_json::json!({
            "email": "test@example.com",
            "2FACode": "123456"
        }),
        serde_json::json!({
            "loginAttemptId": "12345678-1234-1234-1234-123456789012",
            "2FACode": "123456"
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(response.status().as_u16(), 422);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let test_cases = [
        serde_json::json!({
            "email": "invalid-email",
            "loginAttemptId": "12345678-1234-1234-1234-123456789012",
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": "test@example.com",
            "loginAttemptId": "invalid-uuid",
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": "test@example.com",
            "loginAttemptId": "12345678-1234-1234-1234-123456789012",
            "2FACode": "12345"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(response.status().as_u16(), 400);
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    // Sign up a user
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Login to get 2FA code
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    
    let login_response: TwoFactorAuthResponse = response.json().await.unwrap();
    let login_attempt_id = login_response.login_attempt_id;

    // Try to verify with wrong 2FA code
    let verify_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": "999999"
    });
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    // Sign up a user
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Login first time to get first 2FA code
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    
    let login_response: TwoFactorAuthResponse = response.json().await.unwrap();
    let first_login_attempt_id = login_response.login_attempt_id;

    // Login second time to get second 2FA code (this invalidates the first one)
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    
    let login_response: TwoFactorAuthResponse = response.json().await.unwrap();
    let second_login_attempt_id = login_response.login_attempt_id;

    // Try to verify with the first login attempt ID (should fail)
    let verify_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": first_login_attempt_id,
        "2FACode": "123456"
    });
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    // Sign up a user
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Login to get 2FA code
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    
    let login_response: TwoFactorAuthResponse = response.json().await.unwrap();
    let login_attempt_id = login_response.login_attempt_id;

    // Get the 2FA code from the store (this is a bit of a hack for testing)
    let two_fa_store = app.two_fa_code_store.read().await;
    let email = Email::parse(random_email.clone()).unwrap();
    let (stored_login_attempt_id, two_fa_code) = two_fa_store.get_code(&email).await.unwrap();
    drop(two_fa_store);

    // Verify with correct 2FA code
    let verify_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": stored_login_attempt_id.as_ref(),
        "2FACode": two_fa_code.as_ref()
    });
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // Check that auth cookie is set
    let cookies = response.headers().get("set-cookie").unwrap();
    assert!(cookies.to_str().unwrap().contains(JWT_COOKIE_NAME));
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    // Sign up a user
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Login to get 2FA code
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    
    let login_response: TwoFactorAuthResponse = response.json().await.unwrap();
    let login_attempt_id = login_response.login_attempt_id;

    // Get the 2FA code from the store
    let two_fa_store = app.two_fa_code_store.read().await;
    let email = Email::parse(random_email.clone()).unwrap();
    let (stored_login_attempt_id, two_fa_code) = two_fa_store.get_code(&email).await.unwrap();
    drop(two_fa_store);

    // Verify with correct 2FA code first time (should succeed)
    let verify_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": stored_login_attempt_id.as_ref(),
        "2FACode": two_fa_code.as_ref()
    });
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // Try to verify with the same 2FA code again (should fail)
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);
}