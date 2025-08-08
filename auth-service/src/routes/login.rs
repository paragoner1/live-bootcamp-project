// SPRINT 3: Complete login route with JWT token generation
// This was added in Sprint 3 to provide authentication functionality

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{Email, Password, AuthAPIError, Token},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Parse and validate email
    let email = Email::parse(request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Parse and validate password
    let password = Password::parse(request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Validate user credentials
    let user_store = app_state.user_store.read().await;
    user_store.validate_user(&email, &password).await
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Generate JWT token
    let token = Token::new(&email, "your-secret-key")
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    // Return success response with token
    let response = LoginResponse {
        message: "Login successful!".to_string(),
        token: token.as_str().to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

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