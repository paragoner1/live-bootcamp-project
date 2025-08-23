use crate::helpers::{get_random_email, TestApp};
use serde_json;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email 

    // TODO: add more malformed input test cases
    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),  // Missing email
        serde_json::json!({
            "email": random_email.clone(),
            "requires2FA": true
        }),  // Missing password
        serde_json::json!({
            "email": random_email.clone(),
            "password": "password123"
        }),  // Missing requires2FA
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await; // call `post_signup`

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }

    app.clean_up().await;
}