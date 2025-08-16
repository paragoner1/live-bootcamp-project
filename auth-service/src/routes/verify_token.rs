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
    utils::auth::validate_token,
};

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct VerifyTokenResponse {
    pub message: String,
    pub email: String,
}

pub async fn verify_token(
    State(app_state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Parse token from string
    let token = Token::parse(request.token)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Check if token is banned
    let banned_token_store = app_state.banned_token_store.read().await;
    let is_banned = banned_token_store.contains_token(token.as_str()).await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    if is_banned {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // Verify token is valid using the same validation as the auth utils
    let claims = validate_token(token.as_str(), app_state.banned_token_store.clone()).await
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Return success response with user email
    let response = VerifyTokenResponse {
        message: "Token is valid!".to_string(),
        email: claims.sub,
    };

    Ok((StatusCode::OK, Json(response)))
}