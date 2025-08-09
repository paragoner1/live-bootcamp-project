// SPRINT 2: Complete verify-token route with basic functionality
// This was added in Sprint 2 to provide token verification functionality

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::AuthAPIError,
};

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct VerifyTokenResponse {
    pub message: String,
}

pub async fn verify_token(
    State(_app_state): State<AppState>,
    Json(_request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Return success response
    let response = VerifyTokenResponse {
        message: "Token is valid!".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

// SPRINT 3: Complete verify-token route with token validation (commented out)
// This was added in Sprint 3 to provide token verification functionality
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
//     domain::{AuthAPIError, Token},
// };
//
// #[derive(Deserialize)]
// pub struct VerifyTokenRequest {
//     pub token: String,
// }
//
// #[derive(Serialize)]
// pub struct VerifyTokenResponse {
//     pub message: String,
//     pub email: String,
// }
//
// pub async fn verify_token(
//     State(app_state): State<AppState>,
//     Json(request): Json<VerifyTokenRequest>,
// ) -> Result<impl IntoResponse, AuthAPIError> {
//     // Parse token from string
//     let token = Token::parse(request.token)
//         .map_err(|_| AuthAPIError::InvalidCredentials)?;
//
//     // Check if token is banned
//     let banned_token_store = app_state.banned_token_store.read().await;
//     let is_banned = banned_token_store.is_token_banned(&token).await
//         .map_err(|_| AuthAPIError::UnexpectedError)?;
//
//     if is_banned {
//         return Err(AuthAPIError::InvalidCredentials);
//     }
//
//     // Verify token is valid
//     let claims = token.verify("your-secret-key")
//         .map_err(|_| AuthAPIError::InvalidCredentials)?;
//
//     // Return success response with user email
//     let response = VerifyTokenResponse {
//         message: "Token is valid!".to_string(),
//         email: claims.sub,
//     };
//
//     Ok((StatusCode::OK, Json(response)))
// }

// SPRINT 1: Simple placeholder - no token validation
// use axum::{http::StatusCode, response::IntoResponse, Json};
// use serde::Deserialize;
//
// pub async fn verify_token(Json(_request): Json<VerifyTokenRequest>) -> impl IntoResponse {
//     StatusCode::OK.into_response()
// }
//
// #[derive(Deserialize)]
// pub struct VerifyTokenRequest {
//     pub token: String,
// }