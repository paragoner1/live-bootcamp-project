// SPRINT 3: Complete logout route with token banning
// This was added in Sprint 3 to provide logout functionality

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Token},
};

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

pub async fn logout(
    State(app_state): State<AppState>,
    Json(request): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Parse token from string
    let token = Token::parse(request.token)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Verify token is valid
    token.verify("your-secret-key")
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Ban the token
    let mut banned_token_store = app_state.banned_token_store.write().await;
    banned_token_store.ban_token(token).await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    // Return success response
    let response = LogoutResponse {
        message: "Logout successful!".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

// SPRINT 1: Simple placeholder - no token handling
// use axum::{http::StatusCode, response::IntoResponse, Json};
// use serde::Deserialize;
// 
// pub async fn logout(Json(_request): Json<LogoutRequest>) -> impl IntoResponse {
//     StatusCode::OK.into_response()
// }
// 
// #[derive(Deserialize)]
// pub struct LogoutRequest {
//     pub token: String,
// }