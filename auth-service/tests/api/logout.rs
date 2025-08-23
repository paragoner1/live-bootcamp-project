use crate::helpers::TestApp;

#[tokio::test]
async fn logout_returns_200() {
    let mut app = TestApp::new().await;

    // Create a user first
    let random_email = crate::helpers::get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let signup_response = app.post_signup(&signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    // Login to get a valid token
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let login_response = app.post_login(&login_body).await;
    assert_eq!(login_response.status().as_u16(), 200);

    // Logout (no body needed, uses cookies)
    let response = app.post_logout(&serde_json::json!({})).await;

    assert_eq!(response.status().as_u16(), 200);

    app.clean_up().await;
}