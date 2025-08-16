use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME, // New!
    ErrorResponse,
};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    todo!()
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    todo!()
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    todo!()
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail. 
    todo!()
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    // Make sure to assert the auth cookie gets set
    todo!()
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {    
    todo!()
}