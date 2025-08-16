use crate::helpers::TestApp;

#[tokio::test]
async fn verify_token_returns_200() {
    let app = TestApp::new().await;

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

    // Extract token from cookie and verify it
    let cookies = login_response.headers().get("set-cookie").unwrap();
    let cookie_str = cookies.to_str().unwrap();
    let token = cookie_str.split("=").nth(1).unwrap().split(";").next().unwrap();

    let token_body = serde_json::json!({
        "token": token
    });

    let response = app.post_verify_token(&token_body).await;

    assert_eq!(response.status().as_u16(), 200);
}