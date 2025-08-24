// SPRINT 2: Domain-specific error types
// This was added in Sprint 2 to provide proper error handling

use color_eyre::eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthAPIError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    // SPRINT 3: New error types
    #[error("Incorrect credentials")]
    IncorrectCredentials,
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

// SPRINT 1: This was the original empty implementation
// The route handlers just returned StatusCode::OK without any validation 