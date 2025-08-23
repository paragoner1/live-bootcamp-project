use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    let mut app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");

    app.clean_up().await;
}