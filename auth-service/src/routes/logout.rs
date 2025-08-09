// SPRINT 2: Complete logout route with basic functionality
// This was added in Sprint 2 to provide logout functionality

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
pub struct LogoutRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

pub async fn logout(
    State(_app_state): State<AppState>,
    Json(_request): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Return success response
    let response = LogoutResponse {
        message: "Logout successful!".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

// SPRINT 3: Complete logout route with token banning (commented out)
// This was added in Sprint 3 to provide logout functionality
// use axum::{
//     extract::State,
//     http::StatusCode,
//     response::IntoResponse,
//     Json,
// };
// use axum_extra::extract::{cookie, CookieJar};
// use serde::{Deserialize, Serialize};
//
// use crate::{
//     app_state::AppState,
//     domain::{AuthAPIError, Token},
//     utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
// };
//
// #[derive(Deserialize)]
// pub struct LogoutRequest {
//     pub token: String,
// }
//
// #[derive(Serialize)]
// pub struct LogoutResponse {
//     pub message: String,
// }
//
// pub async fn logout(
//     State(state): State<AppState>,
//     jar: CookieJar,
// ) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
//     let cookie = match jar.get(JWT_COOKIE_NAME) {
//         Some(cookie) => cookie,
//         None => return (jar, Err(AuthAPIError::MissingToken)),
//     };
//
//     // Validate token
//     let token = cookie.value().to_owned();
//     let _ = match validate_token(&token, state.banned_token_store.clone()).await {
//         Ok(claims) => claims,
//         Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
//     };
//
//     // Add token to banned list
//     if state
//         .banned_token_store
//         .write()
//         .await
//         .add_token(token.to_owned())
//         .await
//         .is_err()
//     {
//         return (jar, Err(AuthAPIError::UnexpectedError));
//     }
//
//     // Remove jwt cookie
//     let jar = jar.remove(cookie::Cookie::from(JWT_COOKIE_NAME));
//
//     (jar, Ok(StatusCode::OK))
// }

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