// SPRINT 3: Complete login route with JWT token generation
// This was added in Sprint 3 to provide authentication functionality

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{Email, Password, AuthAPIError},
    utils::auth::generate_auth_cookie,
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}

// SPRINT 2: Complete login route with validation (commented out)
// This was added in Sprint 2 to provide login functionality
// use axum::{
//     extract::State,
//     http::StatusCode,
//     response::IntoResponse,
//     Json,
// };
// use serde::{Deserialize, Serialize};
//
// use crate::{
//     app_state::AppState,
//     domain::{Email, Password, AuthAPIError},
// };
//
// #[derive(Deserialize)]
// pub struct LoginRequest {
//     pub email: String,
//     pub password: String,
// }
//
// #[derive(Serialize)]
// pub struct LoginResponse {
//     pub message: String,
// }
//
// pub async fn login(
//     State(app_state): State<AppState>,
//     Json(request): Json<LoginRequest>,
// ) -> Result<impl IntoResponse, AuthAPIError> {
//     // Parse and validate email
//     let email = Email::parse(request.email)
//         .map_err(|_| AuthAPIError::InvalidCredentials)?;
//
//     // Parse and validate password
//     let password = Password::parse(request.password)
//         .map_err(|_| AuthAPIError::InvalidCredentials)?;
//
//     // Validate user credentials
//     let user_store = app_state.user_store.read().await;
//     user_store.validate_user(&email, &password).await
//         .map_err(|_| AuthAPIError::InvalidCredentials)?;
//
//     // Return success response
//     let response = LoginResponse {
//         message: "Login successful!".to_string(),
//     };
//
//     Ok((StatusCode::OK, Json(response)))
// }

// SPRINT 1: Simple placeholder - no validation or authentication
// use axum::{http::StatusCode, response::IntoResponse, Json};
// use serde::Deserialize;
//
// pub async fn login(Json(_request): Json<LoginRequest>) -> impl IntoResponse {
//     StatusCode::OK.into_response()
// }
//
// #[derive(Deserialize)]
// pub struct LoginRequest {
//     pub email: String,
//     pub password: String,
// }