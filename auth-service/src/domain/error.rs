// SPRINT 2: Domain-specific error types
// This was added in Sprint 2 to provide proper error handling

#[derive(Debug)]
pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    // SPRINT 3: New error types
    IncorrectCredentials,
    MissingToken,
    InvalidToken,
    UnexpectedError,
}

// SPRINT 1: This was the original empty implementation
// The route handlers just returned StatusCode::OK without any validation 