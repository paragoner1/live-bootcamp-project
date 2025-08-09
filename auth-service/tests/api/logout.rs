use crate::helpers::TestApp;

#[tokio::test]
async fn logout_returns_200() {
    let app = TestApp::new().await;

    let logout_body = serde_json::json!({
        "token": "test_token"
    });

    let response = app.post_logout(&logout_body).await;

    assert_eq!(response.status().as_u16(), 200);
}