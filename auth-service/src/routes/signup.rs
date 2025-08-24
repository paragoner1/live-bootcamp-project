// SPRINT 2: Complete signup route with validation and storage
// This was added in Sprint 2 to provide full signup functionality

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{Email, Password, User, AuthAPIError},
};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}

#[tracing::instrument(name = "Signup", skip_all, err(Debug))]
pub async fn signup(
    State(app_state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Parse and validate email
    let email = Email::parse(request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Parse and validate password
    let password = Password::parse(request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Create user entity
    let user = User::new(email, password, request.requires_2fa);

    // Store user in database
    let mut user_store = app_state.user_store.write().await;
    user_store.add_user(user).await
        .map_err(|e| match e {
            crate::domain::UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
            _ => AuthAPIError::UnexpectedError,
        })?;

    // Return success response
    let response = SignupResponse {
        message: "User created successfully!".to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// SPRINT 1: Simple placeholder - no validation or storage
// use axum::{http::StatusCode, response::IntoResponse, Json};
// use serde::Deserialize;
// 
// pub async fn signup(Json(_request): Json<SignupRequest>) -> impl IntoResponse {
//     StatusCode::OK.into_response()
// }
// 
// #[derive(Deserialize)]
// pub struct SignupRequest {
//     pub email: String,
//     pub password: String,
//     #[serde(rename = "requires2FA")]
//     pub requires_2fa: bool,
// }