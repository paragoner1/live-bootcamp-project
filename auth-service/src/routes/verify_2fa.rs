// SPRINT 2: Complete verify_2fa implementation
// This was added in Sprint 2 to provide proper 2FA verification
//
// use axum::{http::StatusCode, response::IntoResponse, Json};
// use serde::Serialize;
// 
// pub async fn verify_2fa() -> impl IntoResponse {
//     let response = Json(Verify2FAResponse {
//         message: "2FA verification successful!".to_string(),
//     });
// 
//     (StatusCode::OK, response)
// }
// 
// #[derive(Debug, Serialize, PartialEq)]
// pub struct Verify2FAResponse {
//     pub message: String,
// }

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie,
};

#[derive(Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}

#[derive(Serialize)]
pub struct Verify2FAResponse {
    pub message: String,
}

pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email.clone()) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id.clone()) {
        Ok(id) => id,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let two_fa_code = match TwoFACode::parse(request.two_fa_code.clone()) {
        Ok(code) => code,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let mut two_fa_code_store = state.two_fa_code_store.write().await;

    // Call `two_fa_code_store.get_code`. If the call fails
    // return a `AuthAPIError::IncorrectCredentials`.
    let code_tuple = match two_fa_code_store.get_code(&email).await {
        Ok(tuple) => tuple,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    // Validate that the `login_attempt_id` and `two_fa_code`
    // in the request body matches values in the `code_tuple`. 
    // If not, return a `AuthAPIError::IncorrectCredentials`.
    if code_tuple.0 != login_attempt_id || code_tuple.1 != two_fa_code {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    // Remove the 2FA code from storage after successful validation
    // Call `two_fa_code_store.remove_code(&email)`. If the call fails,
    // return a `AuthAPIError::UnexpectedError`.
    if two_fa_code_store.remove_code(&email).await.is_err() {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    // Generate auth cookie and return it with the response
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };
    let updated_jar = jar.add(auth_cookie);

    // Return a proper JSON response with the cookie
    let response = Verify2FAResponse {
        message: "2FA verification successful!".to_string(),
    };
    (updated_jar, Ok((StatusCode::OK, Json(response))))
}