use axum::{http::StatusCode, response::IntoResponse};

pub async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}